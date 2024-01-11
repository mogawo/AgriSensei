use std::{
    io::{prelude::*, BufReader, Error, Bytes},
    net::{TcpListener, TcpStream}, 
    fs,
    convert::{AsRef, Infallible}, fmt::{Display, Debug}, ops::Deref, slice, borrow::BorrowMut, str::from_utf8,
    // string::String
};

use http::uri::PathAndQuery;
use image::EncodableLayout;
use server::
{
    ThreadPool,
    ServerError::{self, HTTPError, PathError, MimeTypeError},
};

use //External Libs
{
    http::{self, Method, Request, Response, header, method, status, uri},
};

const HOST_ADDRESS: &str = "127.0.0.1:7878";
const ASSEMBLE_VEC_CAPACITY: usize = (1 << 10);

fn main() {
    let listener = TcpListener::bind(HOST_ADDRESS).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {handle_connection(stream);});
    }
}

fn handle_connection(mut stream: TcpStream) -> ServerError {
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

trait Assemble{
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

        println!("{:?}\n", from_utf8(&to_send));
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
        Method::GET => get_request(parts.uri.path()),
        meth => error_response("Unhandled method request", meth.as_str())
    }
}

fn get_request(uri_path: &str) -> Result<http::Response<Vec<u8>>, ServerError>
{
    return match uri_path
    {
        "/" => get_response("pages\\main_page\\index.html"),
        "/script.js" => get_response("pages\\main_page\\script.js"),
        "/style.css" => get_response("pages\\main_page\\style.css"),
        "/images/cog-xxl.png" => get_response("pages\\main_page\\images\\cog-xxl.png"),
        "/favicon.ico" => get_response("pages\\main_page\\images\\favicon.ico"),
        _ => error_response("Requested Path not Found", ServerError::PathError { msg: "Requested Path not Found".to_string(), path: uri_path.to_string() })
    };
}

fn get_response(file_path: &str) -> Result<Response<Vec<u8>>, ServerError>
{
    let (_, extension) = file_path.split_once('.').ok_or(PathError{msg: "Requested File has no extension".to_string(), path: file_path.to_string()})?;
    let mime_type = mime_guess::from_ext(extension).first_raw().ok_or(MimeTypeError{msg: "Requested File Extension has no MIME Type".to_string(), path: file_path.to_string()})?;
    let file_data = match fs::read(file_path){
        Ok(data) => data,
        Err(e) => return Err(ServerError::ResponseError { msg: format!("{}\nCould not open file", e) })
    };

    println!("{}, {}, {}", file_path,mime_type, file_data.len());
    let response = Response::builder()
        .header("Content-Type", mime_type)
        .header("Content-Length", file_data.len())
        .body(file_data);

    Ok(response?)
}

trait Print : Display + Debug {}
impl<T: Display + Debug> Print for T {}

fn error_response<S: Print, E: Print>(msg: S, value: E) -> Result<Response<Vec<u8>>, ServerError>
{
    println!("{}: {}", msg, value);
    let err_file = fs::read("pages\\404.html").unwrap();
    let err_response = Response::builder()
        .status(404)
        .header("Content-Type", "text/html")
        .header("Content-Length", err_file.len())
        .body(err_file);
    Ok(err_response?)
}