
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


/*
    NEXT UP:

    Change from a node server that accepts network operations to a network
    node only that accepts data requests
    */

    #![allow(unused_imports)]

use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::iter::Map;
use std::net::TcpStream;
use std::time::Duration;
use std::{net::TcpListener, thread};
use std::sync::mpsc;
use threadpool::ThreadPool;
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use build_html::*;
    
    fn main() {
        //thread - run the state machine on this thread
        //thread - run some network logic on this thread
        //for each incoming connection - delegate to a threadpool worker depending on type of connection
        //     have a diff function for each type of request
        let args: Vec<String> = env::args().collect();
    
        let port = &args[1];
        let neighbor = &args[2];
    
        let addr = format!("127.0.0.1:{port}");
    
        let (tx, mut rx) = mpsc::channel();
        let tx2 = tx.clone();
    
        let sm_thread = thread::spawn(move || {
            //listen for incoming edits, apply them bitches
            let mut data: HashMap<String, String> = HashMap::new();
    
            while let Some(cmd) = rx.recv().unwrap() {
                use Command::*;
    
                match cmd {
                    Get { key, resp } => {
                        let res = data.get(&key);
                        let _ = resp.send(res.cloned());
                    }
                    Set { key, val, resp } => {
                        let res = data.insert(key, val);
                        let _ = resp.send(res);
                    }
                    GetAll { resp } => {
                        let mut res = Vec::new();
                        for key in data.keys() {
                            let k = String::from(key);
                            let v = String::from(data.get(key).unwrap());
                            res.push((k, v));
                        }
                        let _ = resp.send(res);
                    }
                }
            }
        });
    
        let network_thread = thread::spawn(move || {
            use Command::*;
            //try to come to consensus with the rest of the network
            // go over state machine, calulcate what is consensus
            // when we get a new connection, try and figure out where to send it
            let (oneshot_sender, oneshot_reciever) = oneshot::channel();
            let c = Set {
                key: "name".to_string(), 
                val: "colin".to_string(), 
                resp: oneshot_sender
            };
            tx2.send(Some(c));
        });
    
    
        let n_workers = 1;
        let pool = ThreadPool::new(n_workers);
        //listen for incoming connections, update sm as needed
        println!("{}", addr);
        let listener = TcpListener::bind(addr).unwrap();
    
        println!("listening loop is running");
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            let tmp_tx = tx.clone();
    
            pool.execute(|| {
                handle_connection(stream, tmp_tx);
            })
        }
    
        println!("quitting");
    }
    
    enum Command {
        Get {
            key: String,
            resp: Responder<Option<String>>,
        },
        Set {
            key: String,
            val: String,
            resp: Responder<Option<String>>,
        },
        GetAll {
            resp: Responder<Vec<(String, String)>>,
        },
    }
    
    
    type Responder<T> = oneshot::Sender<T>;
    
    
    fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<Option<Command>>) {
        let buf_reader = BufReader::new(&mut stream);
        let request_vector: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
    
        println!("{}", &request_vector[0]);
        let response = match request_vector[0].as_str() {
            "NETWORK" => network_connection_handler(&request_vector),
            _ => http_request_handler(&request_vector, tx),
        }; 
        stream.write_all(response.as_bytes()).unwrap();
    }
    
    fn http_request_handler(request: &Vec<String>, tx: mpsc::Sender<Option<Command>>) -> String {
    
        println!("found http request");
        let (status_line, html_string) = match request[0].as_str() {
            "GET /data HTTP/1.1" => {
                use Command::*;
                let (one_sender, one_reciever) = oneshot::channel();
                tx.send(Some(GetAll { resp: one_sender }));
                let resp = one_reciever.recv().unwrap();
                let html_string = make_html_from_data(resp)
                    .to_html_string();
                println!("called!");
                ("HTTP/1.1 200 OK", html_string)
            },
            "GET / HTTP/1.1" => {
                let html_string = fs::read_to_string("hello.html").unwrap();
                ("HTTP/1.1 200 OK", html_string)
            },
            
            _ => {
                let html_string = fs::read_to_string("404.html").unwrap();
                ("HTTP/1.1 400 NOT FOUND", html_string)
            },
        };
        
        let length = html_string.len();
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_string}")
    }
    fn network_connection_handler(buf_reader: &Vec<String>) -> String {
        "NETWORK request found, hi!".to_string()
    }
    
    
    fn make_html_from_data(data: Vec<(String, String)>) -> HtmlPage {
        let mut html_string = HtmlPage::new()
                    .with_title("My Page")
                    .with_header(1, "Main Content:")
                    .with_container(
                        Container::new(ContainerType::Article)
                            .with_attributes([("id", "article1")])
                            .with_header_attr(2, "Hello, World", [("id", "article-head")])
                            .with_paragraph("This is a simple HTML demo")
                    );
        for (k, v) in data.iter() {
            html_string.add_paragraph(format!("{k} : {v}"));
        }
        html_string
    }
    
        /*
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
    */
    /*
    
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
                    |l: Result<String, Error>|
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
    */