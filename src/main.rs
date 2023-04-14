

/*
what I want to do:
I want a node-server to be a connection to a network,
that exposes functions to do things on the network
to an application.

First steps:
    connect with another node
    say hi and make sure everything is good there
    maybe set up like a shared notion of a SM and
    add a few attributes
 */

use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::iter::Map;
use std::net::TcpStream;
use std::time::Duration;
use std::{net::TcpListener, thread};
use std::sync::mpsc;
use threadpool::ThreadPool;

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    for line in buf_reader.lines().map(|l| l.unwrap()) {
        println!("{}", line)
    }
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = match &request_line[..] {
    "GET / HTTP/1.1" => http_request_handler(&request_line),
    "NETWORK" => network_connection_handler(
        Vec::new(buf_reader
            .lines()
            .map(
                |l: Result<String>|
                 l.unwrap()
                )
            )
        ),
    }; 


    stream.write_all(response.as_bytes()).unwrap();
}

fn http_request_handler(request: &str) -> String {
    let (status_line, filename) = match request {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

fn network_connection_handler(buf_reader: Vec<String>) -> String {
    "NETWORK request found, hi!".to_string()
}
/*
pub trait Transaction {
    fn new(contents: String) -> Self;
}

trait StateMachine {
    fn new() -> Self;
    fn add_attribute(attribute: Attribute);
}

trait Attribute {
    fn new(contents: T) -> Self;
}
*/