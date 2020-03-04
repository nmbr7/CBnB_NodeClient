extern crate dotenv;
extern crate uuid;

mod api;
mod message;
mod sys_stat;

use api::{client_main, server_main};
use message::Message;
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sys_stat::GetStat;

fn main() -> () {
    let (client_tx, client_rx) = mpsc::channel();
    let (server_tx, server_rx) = mpsc::channel();

    let addr = format!("0.0.0.0:7777");

    //Spawn Server Thread
    let _server_thread = thread::spawn(move || {
        server_main(server_tx, addr);
    });
    //Spawn Client Thread
    let _client_thread = thread::spawn(move || {
        client_main(client_rx);
    });

    //Spawn system status thread
    let client_sys_stat_tx = mpsc::Sender::clone(&client_tx);
    let _sys_stat_thread = thread::spawn(move || {
        let mut stat = sys_stat::Resources::new();
        let msg = Message::<sys_stat::Resources>::register(stat.clone());
        client_tx.send(msg.clone()).unwrap();

        loop {
            thread::sleep(Duration::from_secs(5));
            let mut stat = stat.update_stat();
            let msgu = Message::<sys_stat::StatUpdate>::update(stat.clone());
            client_tx.send(msgu.clone()).unwrap();
        }
    });

    //use a domain instead or the proxy address lookup

    loop {
        let received = server_rx.try_recv();
        match received {
            Ok(s) => {
                println!("Received from Core Server: {}", s);
                //             thread::sleep(Duration::from_secs(1));
                //             let addr = format!("127.0.0.1:7770");
                //             client_tx.send(addr).unwrap();
            }
            Err(_) => (),
        };
    }
    //_client_thread.join().unwrap();
    //_server_thread.join().unwrap();
}
