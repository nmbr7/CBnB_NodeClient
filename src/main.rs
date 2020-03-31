extern crate dotenv;

extern crate librsless;
extern crate uuid;

use std::process::Command;

mod api;
mod message;
mod service;
mod sys_stat;

use api::{client_main, server_main};
use dotenv::dotenv;
use message::NodeMessage;
use std::env;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sys_stat::GetStat;

fn main() -> () {
    dotenv().ok();
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

        let run_mode = env::var("RUN_MODE").expect("RUN_MODE not set");
        match run_mode.as_str() {
            "TEST" => {
                let memtotal = std::str::from_utf8(
                    &Command::new("/node_client/scripts/memlimit.sh")
                        .output()
                        .expect("Error")
                        .stdout,
                )
                .unwrap()
                .trim_matches('\n')
                .to_string();
                let cpuuse = std::str::from_utf8(
                    &Command::new("/node_client/scripts/cpuusage.sh")
                        .output()
                        .expect("Error")
                        .stdout,
                )
                .unwrap()
                .trim_matches('\n')
                .to_string();
                let memuse = std::str::from_utf8(
                    &Command::new("/node_client/scripts/memusage.sh")
                        .output()
                        .expect("Error")
                        .stdout,
                )
                .unwrap()
                .trim_matches('\n')
                .to_string();
                stat.mem.total = format!("{}", memtotal).parse().unwrap();
                stat.cpu.usage = format!("{:.5}", cpuuse).parse().unwrap();
                stat.mem.usage.1 = format!("{:.5}", memuse).parse().unwrap();
            }
            "DEV" => {}
            _ => panic!("Run mode not set"),
        };
        println!("{:?}", stat);

        let msg = NodeMessage::register(stat.clone());
        client_tx.send(msg.clone()).unwrap();

        loop {
            thread::sleep(Duration::from_secs(5));
            let mut stat = stat.update_stat();
            match run_mode.as_str() {
                "TEST" => {
                    let cpuuse = std::str::from_utf8(
                        &Command::new("/node_client/scripts/cpuusage.sh")
                            .output()
                            .expect("Error")
                            .stdout,
                    )
                    .unwrap()
                    .trim_matches('\n')
                    .to_string();
                    let memuse = std::str::from_utf8(
                        &Command::new("/node_client/scripts/memusage.sh")
                            .output()
                            .expect("Error")
                            .stdout,
                    )
                    .unwrap()
                    .trim_matches('\n')
                    .to_string();
                    stat.cpu_usage = format!("{:.5}", cpuuse).parse().unwrap();
                    stat.mem_usage.1 = format!("{:.5}", memuse).parse().unwrap();
                }
                "DEV" => {}
                _ => panic!("Run mode not set"),
            };
            println!("{:?}", stat);
            let msgu = NodeMessage::update(stat.clone());
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
