use std::sync::mpsc;
use std::net::TcpStream;
use std::collections::HashMap;

pub enum Command {
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

pub type Responder<T> = oneshot::Sender<T>;

pub fn run_state_machine(receiver: mpsc::Receiver<Option<Command>>) {
    //listen for incoming edits, apply them bitches
    let mut data: HashMap<String, String> = HashMap::new();

    while let Some(cmd) = receiver.recv().unwrap() {
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
}