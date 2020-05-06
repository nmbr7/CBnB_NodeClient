use dotenv::dotenv;
use log::{debug, info, warn};
use serde_json::{json, Value};

use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use std::io::SeekFrom;

use std::fs::OpenOptions;
use std::io::BufWriter;

use std::fs::File;

use crate::message::{ServiceMessage, ServiceMsgType, ServiceType};
use crate::service::{
    paas::{new_app, start_qemu},
    storage::{storage_read, storage_write},
    Fas, Service,
};
use librsless::msg_parser;
use std::collections::HashMap;

fn server_handler(
    mut stream: TcpStream,
    server_dup_tx: mpsc::Sender<String>,
    service: Arc<Mutex<Service>>,
) -> () {
    let mut buffer = [0; 100_000];
    let no = stream.read(&mut buffer).unwrap();

    let recv_data: ServiceMessage = serde_json::from_slice(&buffer[0..no]).unwrap();
    let json_data = serde_json::from_str(&recv_data.content.as_str()).unwrap();
    debug!("{:?}", json_data);
    //TODO Check the proxy uuid

    match recv_data.msg_type {
        ServiceMsgType::SERVICEINIT => {
            match recv_data.service_type {
                ServiceType::Faas => {
                    let server_res = msg_parser(&mut stream, json_data);

                    stream.write_all("".as_bytes()).unwrap();
                    stream.flush().unwrap();
                    /*
                    let faas_instance =  struct Fas {
                        pub invocations: i32,
                        pub frequency: i32,
                        pub created_on: String,
                        pub status1: String, //published or not
                    }*/
                    {
                        let mut service_instance = service.lock().unwrap();
                        service_instance.faas.metadata.instance_count += 1;
                    }
                }
                ServiceType::Storage => {
                    match json_data["msg_type"].as_str().unwrap() {
                        "read" => {
                            storage_read(&mut stream, json_data);
                            /*{
                                let mut service_instance = service.lock().unwrap();
                                service_instance.storage.metadata.instance_count += 1;
                            }*/
                        }
                        "write" => {
                            storage_write(&mut stream, json_data, service);
                        }
                        _ => {}
                    }
                }
                // Currently the docker deamon runs as root and users can see all the images in VM
                // TODO Restrict user from access the root docker deamon
                ServiceType::Paas => {
                    // TODO handle the deployment directly between proxy and qemu rather than relaying through node
                    match json_data["msg_type"].as_str().unwrap() {
                        "deploy" => {
                            new_app(json_data);
                            {
                                let mut service_instance = service.lock().unwrap();
                                service_instance.faas.metadata.instance_count += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        ServiceMsgType::SERVICEUPDATE => match recv_data.service_type {
            // Incase of deploying the function or updating
            ServiceType::Faas => {
                let server_res = msg_parser(&mut stream, json_data);
                stream.write_all("".as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            // Incase of updating the app version or on bug fixes
            ServiceType::Paas => {}

            // Incase fo update to the file
            ServiceType::Storage => {}
        },
        ServiceMsgType::SERVICEMANAGE => match recv_data.service_type {
            // For setting up the initial faas root directory and related files
            ServiceType::Faas => {}
            // For starting the qemu vm and setting up the qemu server
            ServiceType::Paas => {
                match json_data["msg_type"].as_str().unwrap() {
                    "start" => {
                        info!("Starting Qemu");
                        // TODO Verify the qemu image
                        let pid = start_qemu();
                        println!("Qemu PID is {}", pid);
                    }
                    _ => {}
                }
            }
            // For setting up the storage related directory
            ServiceType::Storage => {}
        },
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

pub fn server_main(
    server_tx: mpsc::Sender<String>,
    addr: String,
    service: Arc<Mutex<Service>>,
) -> () {
    let listener = TcpListener::bind(addr).unwrap();
    info!("Server setup done, waiting for incoming messages");
    /*let mut node_services = Service {
        storages: HashMap::new(),
        faas: HashMap::new(),
        paas: HashMap::new(),
    };
    */

    for stream in listener.incoming() {
        // In case of browser there may be multiple requests for fetching
        // different file in a page
        let stream = stream.unwrap();
        let server_dup_tx = mpsc::Sender::clone(&server_tx);

        let services = Arc::clone(&service);
        //Spawn server request handler thread
        thread::spawn(move || {
            server_handler(stream, server_dup_tx, services);
        });
    }
}

pub fn client_main(client_rx: mpsc::Receiver<String>) -> () {
    dotenv().ok();

    // TODO Create a config file  to handle all the setup

    let run_mode = env::var("RUN_MODE").expect("RUN_MODE not set");
    let server_ip = match run_mode.as_str() {
        "TEST" => String::from("172.28.5.77"),
        "DEV" => String::from("127.0.0.1"),
        _ => panic!("Run mode not set"),
    };
    let server_port = String::from("7779");
    let addr = format!("{}:{}", server_ip, server_port);
    //let client_dup_rx = mpsc::Sender::clone(&client_rx);
    info!("Client waiting for messages to be sent to the server.");
    for received in client_rx {
        let stream = TcpStream::connect(&addr).unwrap();
        thread::spawn(move || {
            client_handler(stream, received);
        });
    }
}
