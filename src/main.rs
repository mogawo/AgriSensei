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

mod components;
use components::*;


const HOST_ADDRESS: &str = "127.0.0.1:7878";
// const TEST_NAMES: [&str; 20] = ["Yareli", "Sophie", "Winston", "Norman", 
//                                 "Kimberly", "Kara", "Juan", "Billy", 
//                                 "Braulio", "Damien", "Ezra", "Margarita", 
//                                 "Gisselle", "Leeann", "Davis", "Alex", 
//                                 "Justin", "Kenna", "Jorden", "Valentin"];

const TEST_NAMES: [&str; 20] = ["Yareli", "Sophie", "Winston", "Norman", 
                                "Kimberly", "Kara", "Juan", "Billy", 
                                "Braulio", "Damien", "Ezra", "Margarita", 
                                "Gisselle", "Leeann", "Davis", "Alex", 
                                "Justin", "Kenna", "Jorden", "Remy"];

                   
fn main(){
    // run();
    UserProfile::pull_user(1, Some(&[1,2]));
    
}

fn database_testing(){
    Database::new();
    let name_numerate:&mut  Vec<(u64, &str)> = &mut Vec::new();

    for name in TEST_NAMES {
        Database::new_user(name);
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

