
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

//use std::io::BufReader;
//use std::io::prelude::*;
//use std::fs;
//use std::iter::Map;
//use std::net::TcpStream;
//use std::time::Duration;
use std::{net::TcpListener, thread};
use std::sync::mpsc;
use threadpool::ThreadPool;
use std::env;
use std::collections::HashMap;
mod state_machine;
use state_machine::*;
//use std::sync::Arc;
//use build_html::*;
//use state_machine::*;

fn main() {
    //thread - run the state machine on this thread
    //thread - run some network logic on this thread
    //for each incoming connection - delegate to a threadpool worker depending on type of connection
    //     have a diff function for each type of request
    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let neighbor = &args[2];

    let addr = format!("127.0.0.1:{port}");

    let (sender, receiver) = mpsc::channel();

    let sm_thread = thread::spawn(move || {
        run_state_machine(receiver);
    });

    let n_workers = 1;
    let pool = ThreadPool::new(n_workers);
    //listen for incoming connections, update sm as needed
    println!("{}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    println!("listening loop is running");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let stream_sender = sender.clone();

        pool.execute(|| {
            handle_connection(stream, stream_sender);
        })
    }

    println!("quitting");
}

fn handle_connection(stream: TcpStream, sender: mpsc::Sender<Option<Command>>) {
    todo!;
}
