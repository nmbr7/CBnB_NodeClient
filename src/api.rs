use dotenv::dotenv;
use serde_json::{ json, Value};

use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::message::{ServiceMessage, ServiceMsgType, ServiceType};
use crate::service::{Fas, Service};
use librsless::msg_parser;
use std::collections::HashMap;


fn server_handler(mut stream: TcpStream, server_dup_tx: mpsc::Sender<String>) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    //let now = Instant::now();
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 100_000];
    let no = stream.read(&mut buffer).unwrap();
    //let buf = buffer.trim_matches(char::from(0));
    //let mut reader = BufReader::new(stream);
    //let lines = reader.lines();
    //let v = lines.map(|l| l.expect("Parse Fail")).collect()

    //let r = format!("{}", String::from_utf8_lossy(&buffer[0..no]));
    //let a = buffer[0..no].split("_:_").map(|l| l.to_string()).collect::<Vec<String>>();
    let recv_data: ServiceMessage = serde_json::from_slice(&buffer[0..no]).unwrap();
    let json_data = serde_json::from_str(&recv_data.content.as_str()).unwrap();
    //println!("{:?}", json_data);

    match recv_data.msg_type {
        ServiceMsgType::SERVICEINIT => {
            match recv_data.service_type {
                ServiceType::Faas => {
                    let server_res = msg_parser(&mut stream, json_data);

                    stream.write_all("".as_bytes()).unwrap();
                    stream.flush().unwrap();
                    let FasService = Fas {
                        service_id: String::from("uuid"),
                    };

                    /*
                    pub struct Service {
                        pub vms: HashMap<String, Vm>,
                        pub storages: HashMap<String, Storage>,
                        pub dockersapps: HashMap<String, Docker>,
                        pub faas: HashMap<String, Fas>,
                    }*/
                    // let Services = Service {

                    // }
                }
                ServiceType::Storage => {
                    match json_data["msg_type"].as_str().unwrap(){
                        "read" => {
                            let offset = &json_data["offset"];
                            let size = &json_data["size"];
                            let block = &json_data["blockno"];

                            //seek to the file and read the chunk
                        }
                        "write" => {

                            //write to any free block and return the details

                            let data = json!({
                                "blockno": "no",
                                "offset": "offset",
                                "c_hash": "hash",
                                "block_hash": "bhash",
                            }).to_string();
                            stream.write_all(data.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }
                        _ => {},
                    }
                }
                // Currently the docker deamon runs as root and users can see all the images in VM 
                // TODO Restrict user from access the root docker deamon
                ServiceType::Paas => {
                    match json_data["msg_type"].as_str().unwrap(){
                        "start" => {
                            let lang = &json_data["lang"];
                            // SSH to VM using the private key

                            // Generate a public/private key pair for the user PaaS instance 
                            // Send the public key and user uuid to the VM
                            // Create a new user of the respective uuid

                            // Return the private key to the PaaS user

                        }
                        _ => {},
                    }
                }
            }
        }
    }

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
    let mut node_services = Service{
        vms: HashMap::new(),
        storages: HashMap::new(),
        dockerapps: HashMap::new(),
        faas: HashMap::new(),
    };

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
        "DEV" => String::from("172.28.5.77"),
        "TEST" => String::from("127.0.0.1"),
        _ => panic!("Run mode not set"),
    };
    let server_port = String::from("7779");
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
