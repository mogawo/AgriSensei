use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}
};

use crate::{rest::post::PostMessage, ServerError::{self, *}};
use crate::rest::get::GetMessage;
use crate::message::Message;
use http::{self, Method, Request, Response};


const ASSEMBLE_VEC_CAPACITY: usize = 1 << 10;

pub  fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError<'static>> {
    let mut buf_reader = BufReader::new(&mut stream);
    
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req: httparse::Request<'_, '_> = httparse::Request::new(&mut headers);

    let req_buffer = buf_reader.fill_buf().unwrap();
    let req_status = req.parse(&req_buffer).unwrap();
    if req_status.is_partial() {
        return Err(ServerError::ThreadError("Partial Request, Retry Request"));
    }; //TODO Handle partial request
    
    let body_offset = req_status.unwrap();
    let req_body = &mut String::new();
    let _ = (&req_buffer[body_offset..]).read_to_string(req_body);
    let req_body: String = req_body.clone();

    //convert httparse struct -> http struct
    let mut req_builder =  Request::builder()
        .method(req.method.unwrap())
        .uri(format!("{}", req.path.unwrap()));

    for head in headers
    {
        if head != httparse::EMPTY_HEADER
        {
            let key = head.name;
            let value = head.value;
            req_builder = req_builder.header(key, value);
        }
    }

    let built_request = req_builder.body(req_body)?;

    let built_response = handle_response(built_request)?;
    let response_bytes = built_response.assemble()?;

    stream.write_all(&response_bytes).unwrap();
    stream.flush().unwrap();
    Ok(())
}

pub trait Assemble{
    fn assemble(self) -> Result<Vec<u8>,  ServerError<'static>>;
    fn strip_cast<'h>(&'h self, line: &'h mut String) -> String;
}
impl Assemble for Response<Vec<u8>>{
    //Assembles the Response key/values to vec of bytes
    //Mostly to used to send response as bytes to TCPStream
    fn assemble(self) -> Result<Vec<u8>, ServerError<'static>>{
        let mut to_send = Vec::<u8>::with_capacity(ASSEMBLE_VEC_CAPACITY);

        let mut version = format!("{:?}\r\n", self.version());
        to_send.extend(self.strip_cast(&mut version).bytes());

        let mut status = format!("{:?}\r\n", self.status());
        to_send.extend(self.strip_cast(&mut status).bytes());

        for (name, value) in self.headers(){
            let mut header = format!("{:?}: {:?}\r\n", name, value);
            to_send.extend(self.strip_cast(&mut header).bytes());
        }
        
        to_send.extend("\r\n".bytes());

        to_send.extend(self.body());
        Ok(to_send)
    }

    fn strip_cast<'h>(&self, line: &'h mut String) -> String{
        let stripped = line.chars()
            .filter(|c| {*c != '"'})
            .collect();
        stripped
    }
}

 fn handle_response(req: http::Request<String>) -> Result<http::Response<Vec<u8>>, ServerError<'static>>
{
    match req.method()
    {
        &Method::GET => GetMessage::process_request(req),
        &Method::POST => PostMessage::process_request(req),
        _ => GetMessage::error_response(MessageError("=> ResponseError: Unhandled method request"))
    }
}
