use std::{
    net::{TcpStream},
    io::{prelude::*, BufReader}
};

use crate::ServerError::{self, *};
use crate::rest::get;
use http::{self, Method, Request, Response};


const ASSEMBLE_VEC_CAPACITY: usize = (1 << 10);

pub fn handle_connection(mut stream: TcpStream) -> ServerError {
    let mut buf_reader = BufReader::new(&mut stream);
    
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req: httparse::Request<'_, '_> = httparse::Request::new(&mut headers);

    let req_buffer = buf_reader.fill_buf().unwrap();
    let req_status = req.parse(&req_buffer).unwrap();
    if req_status.is_partial() {
        return ServerError::ThreadError { msg: "Partial Request, Retry Request".to_string()};
    }; //TODO Handle partial request
    
    let body_offset = req_status.unwrap();
    let req_body = &req_buffer[body_offset..];

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

    let built_request = match req_builder.body(req_body){
        Ok(built_req) => built_req,
        Err(err) => return err.into()
    };

    let built_response = match handle_response(built_request){
        Ok(handled_resp) => handled_resp,
        Err(err) => return err
    };

    let response_bytes = match built_response.assemble(){
        Ok(resp_bytes) => resp_bytes,
        Err(err) => return err
    };

    stream.write_all(&response_bytes).unwrap();
    stream.flush().unwrap();
    ServerError::ThreadError { msg: "Thread has been closed".to_string() }
}

pub trait Assemble{
    fn assemble(&self) -> Result<Vec<u8>, ServerError>;
    fn strip_cast<'h>(&self, line: &'h mut String) -> String; //Gets rid of quotes 
}
impl Assemble for Response<Vec<u8>>{
    //Assembles the Response key/values to vec of bytes
    //Mostly to used to send response as bytes to TCPStream
    fn assemble(&self) -> Result<Vec<u8>, ServerError>{
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

fn handle_response(req: http::Request<&[u8]>) -> Result<http::Response<Vec<u8>>, ServerError>
{
    let (parts, body) = req.into_parts();
    match parts.method
    {
        Method::GET => get::request(parts.uri.path()),
        meth => get::error_response("Unhandled method request", meth.as_str())
    }
}
