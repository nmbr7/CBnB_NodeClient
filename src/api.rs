use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::thread;


fn server_handler(mut stream: TcpStream, server_dup_tx: mpsc::Sender<String>) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"New NodeClient Registered";
    if buffer.starts_with(get) {
        println!("recieved");
        server_dup_tx
            .send(format!(
                "{:?}",
                str::from_utf8(&buffer)
                    .unwrap()
                    .split("--")
                    .collect::<Vec<&str>>()[0]
            ))
            .unwrap();
    } else {
        server_dup_tx
            .send(format!(
                "{:?}",
                str::from_utf8(&buffer)
                    .unwrap()
                    .split("--")
                    .collect::<Vec<&str>>()[0]
            ))
            .unwrap();
    }
}


fn client_handler(mut stream: TcpStream) -> () {
    let get = b"[[Register Node]]--";
    stream.write(get).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"New NodeClient Registered--";
    if buffer.starts_with(get) {
        println!(
            "Recieved - {}",
            str::from_utf8(&buffer)
                .unwrap()
                .split("--")
                .collect::<Vec<&str>>()[0]
        );
        // server_dup_tx.send(format!("{:?}", str::from_utf8(&buffer).unwrap())).unwrap();
    }
    //println!("{:?}",stream);
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
    //let client_dup_rx = mpsc::Sender::clone(&client_rx);
    println!("Node Client waiting for message requests to be sent to the core server.. ");
    for received in client_rx {
        let stream = TcpStream::connect(received).unwrap();
        thread::spawn(move || {
            client_handler(stream);
        });
    }
}
