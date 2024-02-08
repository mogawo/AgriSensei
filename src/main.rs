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
pub mod server_error;
use server_error::ServerError;

mod thread_pool;
use thread_pool::ThreadPool;

pub mod message;
use message::Message;
mod rest{
    pub mod get;
    pub mod post;
}
// use rest::get;

mod database;
use database::Database;

mod handler;
use handler::handle_connection;


const HOST_ADDRESS: &str = "127.0.0.1:7878";
const TEST_NAMES: [&str; 20] = ["Yareli", "Sophie", "Winston", "Norman", 
                                "Kimberly", "Kara", "Juan", "Billy", 
                                "Braulio", "Damien", "Ezra", "Margarita", 
                                "Gisselle", "Leeann", "Davis", "Alex", 
                                "Justin", "Kenna", "Jorden", "Valentin"];

                      
fn main(){
    // run();
    let mut main = Database::new();
    for name in TEST_NAMES {
        match Database::new_user(name) {
            Some(userID) => todo!("idk do something with this later seems important"),
            None => println!("{}", format!("Username: \"{name}\" has already been taken!"))
        }
    }
}

fn run(){
    let listener = TcpListener::bind(HOST_ADDRESS).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {handle_connection(stream);});
    }
}

