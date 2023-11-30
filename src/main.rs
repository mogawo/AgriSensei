use std::{
    io::{prelude::*, BufReader, Cursor},
    net::{TcpListener, TcpStream}, fs, str::from_utf8, error,
    path::Path
};

use server::ThreadPool;

use http::{self, header, Extensions};
use httparse::{self, Status};

use mime_guess::{self, from_path};

use image::{self, io::Reader as ImageReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection<'s>(mut stream: TcpStream) {
//Stream --> get a array of bytes --> into http parser --> get headers
    let mut buf_reader = BufReader::new(&mut stream);

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let req_status = req.parse(buf_reader.fill_buf().unwrap()).unwrap();
    
    let resp: &[u8];
    if req_status.is_complete() 
    {
        resp = handle_response(&req);

    }
    else {
        resp = error_response("Partial Request Error"); 
        // println!("{:?}\r\n\r\n{}", buf_reader, from_utf8(buf_reader.buffer()).unwrap());
    }

    stream.write_all(resp).unwrap();

    
}

fn handle_response<'r, 'a>(req: &'a httparse::Request) -> Vec<u8>
{
    if req.method.unwrap() == "GET"
    {
        match req.path.unwrap(){
            "/" => return build_response("pages\\main_page\\index.html"),
            "/style.css" => return build_response("pages\\main_page\\style.css"),
            "/script.js" => return build_response( "pages\\main_page\\script.js"),
            "/images/cog-xxl.png" => return build_response( "pages\\main_page\\images\\cog-xxl.png"),
            err_path => return error_response(format!("Request Path cannot be found: {err_path}")),
        }
    }
    else {
        return error_response(format!("Unhandled HTTP Method {}", req.method.unwrap()).as_str());
    }   
}

fn build_response(path: &str) -> Vec<u8>
{
    let file_path: &Path = Path::new(path);
    println!("{:?}: {}", file_path, path);
    let mime_type: Option<&'static str> = mime_guess::from_path(file_path).first_raw();
    println!("{:?}: {:?}", file_path.file_name().unwrap(), mime_type);
    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type.unwrap()) 
        .body(fs::read(file_path).unwrap()).unwrap();

    let (parts, body) = response.into_parts();
    let http::response::Parts {status, version, headers, ..} = parts;
    let status_line = format!("{version:?} {status}");
    let mut str_headers = String::new();
    for (key, value) in headers.iter()
    {
        str_headers.push_str(format!("{}: {}\r\n", key.as_str(), value.to_str().unwrap()).as_str());
    }

    let head_fmt = format!("{status_line}\r\n{str_headers}\r\n\r\n");
    let mut head  = head_fmt.as_bytes().to_vec();
    return [head, body].concat();
}


fn error_response<'r>(err_msg: String)-> Vec<u8>
{
    println!("{}", err_msg);
    build_response("pages\\404.html")
}
