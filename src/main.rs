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

use std::{
    net::{
        TcpListener, 
        TcpStream,
    },
    thread,
    sync::mpsc,
    env,
    collections::HashMap,
};
use threadpool::ThreadPool;
mod state_machine;
mod general;
mod actions;
use crate::state_machine::follower::*;
use crate::actions::request_handler::*;

fn main() {
    /*
    responsible for starting node and delegating running
    starts state machine thread and state machine manager on that thread
    initializes a threadpool
    listens for connections and delegates evaluation of those to the threadpool
     */

    //---------------------------------------------
    // set up state machine
    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let neighbor = &args[2];

    let addr = format!("127.0.0.1:{port}");

    let (sender, receiver) = mpsc::channel();

    let sm_thread = thread::spawn(move || {
        run_state_machine(receiver);
    });

    //---------------------------------------------
    // set up threadpool

    let n_workers = 1;
    let pool = ThreadPool::new(n_workers);
    //listen for incoming connections, update sm as needed

    //---------------------------------------------
    // start listening for connections and delegating
    println!("{}", format!("listening loop is running on {}", addr.clone()));
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let stream_sender = sender.clone();

        pool.execute(|| {
            handle_connection(stream, stream_sender);
        })
    }

    println!("quitting");
}
