mod api;

use api::{client_main, server_main};
//use std::env;
use std::sync::mpsc;
//use std::time::Duration;
use std::thread;
mod sys_stat;

fn main() -> () {
    let (client_tx, client_rx) = mpsc::channel();
    let (server_tx, server_rx) = mpsc::channel();

//    let args: Vec<String> = env::args().collect();
//    if args.len() < 2{
//        panic!("Enter Port No");
//    } 
//    let addr = format!("127.0.0.1:{}", args[1]);
    let addr = format!("0.0.0.0:7777");

    //Spawn Server Thread
    let _server_thread = thread::spawn(move || {
        server_main(server_tx, addr);
    });
    //Spawn Client Thread
    let _client_thread = thread::spawn(move || {
        client_main(client_rx);
    });

    let server_ip = String::from("172.28.5.1");
    let server_port = String::from("7778");
    let addr = format!("{}:{}",server_ip,server_port);
    client_tx.send(addr).unwrap();

    //for i in 1..5 {
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
      //  break;
    }
    //_client_thread.join().unwrap();
    //_server_thread.join().unwrap();
}
