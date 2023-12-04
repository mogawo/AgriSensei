use std::{
    io::{prelude::*, BufReader, Cursor},
    net::{TcpListener, TcpStream}, fs, str::from_utf8, error,
    path::Path
};

use server::ThreadPool;

use http::{self, header, StatusCode, Extensions};
use httparse::{self, Status};

use mime_guess::{self, from_ext};

use image::{self, io::Reader as ImageReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // pool.execute(|| handle_connection(stream));
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
//Stream --> get a array of bytes --> into http parser --> get headers
    let mut buf_reader = BufReader::new(&mut stream);

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let req_status = req.parse(buf_reader.fill_buf().unwrap()).unwrap();
    
    let resp = if req_status.is_complete() {
        handle_response(&req)
    } else {
        error_response("Partial Request Error", StatusCode::NOT_FOUND)
    };

    stream.write(&resp).unwrap();

    
}

fn handle_response(req: &httparse::Request) -> Vec<u8>
{
    if req.method.unwrap() == "GET"
    {
        match req.path.unwrap(){
            "/" => return build_response("pages\\main_page\\index.html"),
            "/style.css" => return build_response("pages\\main_page\\style.css"),
            "/script.js" => return build_response( "pages\\main_page\\script.js"),
            "/images/cog-xxl.png" => return build_response( "pages\\main_page\\images\\cog-xxl.png"),
            "/favicon.ico" => return build_response("pages\\main_page\\images\\icons8-potted-plant-96.png"),
            err_path => return error_response(format!("Request Path cannot be found: {err_path}"), StatusCode::NOT_FOUND),
        }
    }
    else {
        return error_response(format!("Unhandled HTTP Method {}", req.method.unwrap()), StatusCode::NOT_FOUND);
    }   
}

fn build_response<S: AsRef<str>>(path: S) -> Vec<u8>
{
    let path = path.as_ref();
    let file_path: &Path = Path::new(path);
    let ext = file_path.extension().unwrap().to_str();
    let mime_type = from_ext(ext.unwrap()).first().unwrap();
    let file_data = fs::read(file_path).unwrap();
    let file_length = file_data.len();

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type.as_ref())
        .header(header::CONTENT_LENGTH, file_length) 
        .body(file_data).unwrap();

    let (parts, body) = response.into_parts();
    let http::response::Parts {status, version, headers, ..} = parts;
    let status_line = format!("{version:?} {status}");
    let mut str_headers = String::new();
    for (key, value) in headers.iter()
    {
        str_headers.push_str(format!("{}: {}\r\n", key.as_str(), value.to_str().unwrap()).as_str());
    }

    let head_fmt = format!("{status_line}\r\n{str_headers}\r\n\r\n");
    let head  = head_fmt.as_bytes().to_vec();
    
    return [head, body].concat();
}


fn error_response<S: AsRef<str>>(err_msg: S, status: http::StatusCode)-> Vec<u8>
{
    let err_path = Path::new("pages\\404.html");
    let err_data = fs::read(err_path).unwrap();
    let err_length = err_data.len();

    let response = http::Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/html")
        .header(header::CONTENT_LENGTH, err_length) 
        .body(err_data).unwrap();
    println!("{}", err_msg.as_ref());
    
    "TEST COMMIT TO ENSURE I DIDNT JUST ERASE THE MAIN BRANCH"
}
