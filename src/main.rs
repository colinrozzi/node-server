

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

#![allow(unused)]
#![allow(dead_code )]

use tokio::net::{TcpListener, TcpStream};
use std::env;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use bytes::Bytes;
use std::collections::HashMap;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let port = &args[0];
    let neighbor = &args[1];

    let addr = format!("127.0.0.1:{port}");

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut data = HashMap::new();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    data.get(&key);
                }
                Set { key, val } => {
                    data.insert(key, val);
                }
            }
        }
    });
}

enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}

/*
fn network_process(addr: String) {
    print!("{}", "hey, this is where the network process goes")
}

async fn listening_process(addr: String) {

    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}
*/
/*

    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        println!("{}", "got a connection!");
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

 //   let request_line = buf_reader.lines().next().unwrap().unwrap();
 //   let mut request_vector = Vec::new();

    let request_vector: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_header: &str = "GET / HTTP/1.1";
    let network_header: &str = "NETWORK";
    println!("{}", &request_vector[0]);

    let response = match &request_vector[0] {
        http_header => http_request_handler(&request_vector),
        network_header => network_connection_handler(&request_vector),
    }; 

    stream.write_all(response.as_bytes()).unwrap();
}

fn http_request_handler(request: &Vec<String>) -> String {
    println!("found http request");
    let (status_line, filename) = match request {
        _ => ("HTTP/1.1 200 OK", "hello.html"),
        //"GET /sleep HTTP/1.1" => {
        //    thread::sleep(Duration::from_secs(5));
        //    ("HTTP/1.1 200 OK", "sleep.html")
        //}
        
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

fn network_connection_handler(buf_reader: &Vec<String>) -> String {
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
*/