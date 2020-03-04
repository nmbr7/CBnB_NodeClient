use dotenv::dotenv;
use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn server_handler(mut stream: TcpStream, server_dup_tx: mpsc::Sender<String>) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 512];
    //let now = Instant::now();
    let no = stream.read(&mut buffer).unwrap();
    //let secs = now.elapsed().as_secs_f64();
    //println!("Speed : {} Mbps ",(bytes_recvd as f64/((1024*1024) as f64))/(secs as f64));
}

fn client_handler(mut stream: TcpStream, msg: String) -> () {
    //println!("{}",msg);
    stream.write(msg.as_bytes()).unwrap();
    stream.flush().unwrap();
    /*
        let mut buffer = [0; 512];
        let now = Instant::now();
        let no = stream.read(&mut buffer).unwrap();
        let secs = now.elapsed().as_secs_f64();

    */
}

pub fn server_main(server_tx: mpsc::Sender<String>, addr: String) -> () {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Node Server waiting for incomming messages.. ");
    for stream in listener.incoming() {
        // In case of browser there may be multiple requests for fetching
        // different file in a page
        let stream = stream.unwrap();
        let server_dup_tx = mpsc::Sender::clone(&server_tx);
        //Spawn server request handler thread
        thread::spawn(move || {
            server_handler(stream, server_dup_tx);
        });
    }
}

pub fn client_main(client_rx: mpsc::Receiver<String>) -> () {
    dotenv().ok();

    // TODO Create a config file  to handle all the setup

    let run_mode = env::var("RUN_MODE").expect("RUN_MODE not set");
    let server_ip = match run_mode.as_str() {
        "TEST" => String::from("172.28.5.1"),
        "DEV" => String::from("127.0.0.1"),
        _ => panic!("Run mode not set"),
    };
    let server_port = String::from("7778");
    let addr = format!("{}:{}", server_ip, server_port);
    //let client_dup_rx = mpsc::Sender::clone(&client_rx);
    println!("Node Client waiting for message requests to be sent to the core server.. ");
    for received in client_rx {
        let stream = TcpStream::connect(&addr).unwrap();
        thread::spawn(move || {
            client_handler(stream, received);
        });
    }
}
