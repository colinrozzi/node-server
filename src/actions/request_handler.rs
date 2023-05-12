use std::{
    fs,
    net::{
        TcpListener, 
        TcpStream,
    },
    thread,
    sync::mpsc,
    env,
    collections::HashMap,
    io::{BufReader,
        BufRead,
        Write,
    },
};
use build_html::*;

use crate::state_machine::follower::*;

pub fn handle_connection(mut stream: TcpStream, sender: mpsc::Sender<Option<Command>>) {
    let request = parse_tcpstream(&stream);

    match request[0].as_str() {
        "NETWORK" => handle_network_request(stream, sender, request),
        _ => handle_http_request(stream, sender, request),
    };
}

fn handle_network_request(mut stream: TcpStream, sender: mpsc::Sender<Option<Command>>, request: Vec<String>) {
    todo!()
}

fn handle_http_request(mut stream: TcpStream, sender: mpsc::Sender<Option<Command>>, request: Vec<String>) {
    let (status_line, html_string) = match request[0].as_str() {
        "GET /data HTTP/1.1" => {
            use Command::*;
            let (one_sender, one_reciever) = oneshot::channel();
            sender.send(Some(GetAll { resp: one_sender }));
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
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_string}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_tcpstream(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let request_vector: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    request_vector
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