use std::{
    io::{prelude::*, BufReader, Error},
    net::{TcpListener, TcpStream}, 
    fs,
    convert::{AsRef, Infallible}, fmt::{Display, Debug}, ops::Deref,
    // string::String
};

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

fn handle_connection<'s>(mut stream: TcpStream) -> ServerError {
    let mut buf_reader = BufReader::new(&mut stream);
    
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req: httparse::Request<'_, '_> = httparse::Request::new(&mut headers);
    let req_status = req.parse(buf_reader.fill_buf().unwrap()).unwrap();
    if req_status.is_partial() {
        return ServerError::ThreadError { msg: "Partial Request, Retry Request" };
    }; //TODO Handle partial request
    let body_offset = req_status.unwrap();
    let req_body = &buf_reader.buffer()[body_offset..];
    //                                        ^^^^^^^^^^^
    //BufReader::fill_buf docs advise to 'consume' to tell IO source to skip the amount of bytes that filled the buf
    //That is if we are dealing with a singular IO stream.
    //  e.g. IO stream is locked and 'consume' is ASSUMED to unlock and incremented for # of bytes
    //This server is multi-threaded and TcpStream continually sends bytes so no need for consume

    //convert httparse struct -> http struct
    let req_builder =  Request::builder()
        .method(req.method.unwrap())
        .uri(format!("{}", req.path.unwrap()));

    for head in headers
    {
        if head != httparse::EMPTY_HEADER
        {
            let key = head.name;
            let value = head.value;
            req_builder.header(key, value);
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
        
    stream.write_all(response_bytes);
    
    ServerError::ThreadError { msg: "Thread has been closed" }
}
//TODO turn Response into Vec<u8> of bytes
trait Assemble{
    fn assemble(&self) -> Result<&[u8], ServerError>; 
}
impl Assemble for Response<Vec<u8>>{
    //Assembles the Response key/values to vec of bytes
    //Mostly to used to send response as bytes to TCPStream
    fn assemble(&self) -> Result<&[u8], ServerError>{
        let mut to_send = Vec::<u8>::with_capacity(ASSEMBLE_VEC_CAPACITY);
        let (parts, body) = self.into_parts();
        // if let parts_version = {
        //     Ok()
        // };
        to_send.extend(format!("{:?}", parts.version).bytes());
        to_send.extend(format!("{:?}", parts.status).bytes());
        for (name, value) in parts.headers{
            to_send.extend(format!("{:?}: {:?}\r\n", name, value).bytes());
        }
        Ok(&to_send)
    }
}

fn handle_response(req: http::Request<&[u8]>) -> Result<http::Response<Vec<u8>>, ServerError>
{
    match *req.method()
    {
        Method::GET => get_request(req),
        meth => error_response("Unhandled method request", meth.as_str())
    }
}

fn get_request(req: Request<&[u8]>) -> Result<http::Response<Vec<u8>>, ServerError>
{
    let (parts, body) = req.into_parts();
    let uri_path: String = parts.uri.to_string();
    let response = match uri_path.as_ref().map(String::as_ref)
    {
        "\\" => get_response("pages\\main_page\\index.html"),
        "\\script.js" => get_response("pages\\main_page\\index.html"),
        "\\style.css" => get_response("pages\\main_page\\index.html"),
        "\\cog-xxl.png" => get_response("pages\\main_page\\images\\cog-xxl.png"),
        "\\icons8-potted-plant-16.png" => get_response("pages\\main_page\\index.html"),
        off_path => error_response("Requested Path not Found", ServerError::PathError { msg: "Requested Path not Found", path: off_path })
    };
    response
}

fn get_response(file_path: &'static str) -> Result<Response<Vec<u8>>, ServerError>
{
    // let file_path = path.clone();

    let (_, extension) = file_path.split_once('.').ok_or(PathError{msg: "Requested File has no extension", path: file_path})?;
    let mime_type = mime_guess::from_ext(extension).first_raw().ok_or(MimeTypeError{msg: "Requested File Extension has no MIME Type", path: file_path})?;
    let file_data = fs::read(file_path).unwrap();

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