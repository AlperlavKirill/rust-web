extern crate data_encoding;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use postgres::Error as PostgresError;

use crate::data::responses::*;
use crate::data::user::{handlers::*, user::setup_users_db};

pub mod data;

const DB_URL: &str = env!("DATABASE_URL");

fn main() {
    if let Err(e) = set_database() {
        println!("Error with db url: {}", DB_URL);
        println!("Error setting database: {}", e);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8080".to_string()).unwrap();
    println!("Server started at port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                print!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if request.starts_with("POST /register") => handle_register_request(r),
                r if request.starts_with("POST /login") => handle_login_request(r),
                r if request.starts_with("GET /users/") => handle_get_user_request(r),
                r if request.starts_with("GET /users") => handle_get_all_users_request(r),
                r if request.starts_with("PUT /users/") => handle_update_user_request(r),
                r if request.starts_with("DELETE /users/") => handle_delete_user_request(r),

                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string())
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

fn set_database() -> Result<(), PostgresError> {
    setup_users_db()
}
