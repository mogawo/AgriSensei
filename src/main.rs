use std::net::TcpListener;

//Notes on seperating files because I forget
//  Directories need a seperate <dir-name>.rs file in \src\
//  In main.rs, use mod to navigate the file path 
//  mod dir{
//         pub mod a;
//         pub mod b;
// }
// Then use 'use' actually use the modules
// Modules using another module, just use crate::<file-path>
mod server_error;
use server_error::ServerError;

mod thread_pool;
use thread_pool::ThreadPool;

mod rest{
    pub mod get;
}
// use rest::get;

mod database;
use database::Database;

mod handler;
use handler::handle_connection;


const HOST_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    // run();
    let mut db = Database::new().unwrap();
    db.newUser("Johnny");
}

fn run(){
    let listener = TcpListener::bind(HOST_ADDRESS).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {handle_connection(stream);});
    }
}

