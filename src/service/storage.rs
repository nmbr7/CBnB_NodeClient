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

use crate::service::Service;

pub fn storage_read(stream: &mut TcpStream, json_data: Value) {
    let offset = json_data["metadata"]["offset"].as_u64().unwrap();
    let size = json_data["metadata"]["size"].as_u64().unwrap();
    let index = json_data["metadata"]["index"].as_u64().unwrap();
    let block = json_data["metadata"]["blockno"]
        .as_str()
        .unwrap()
        .to_string();

    let mut file = File::open("./storage.bin").unwrap();

    let of = file.seek(SeekFrom::Start(offset)).unwrap();

    //let mut contents = vec![];
    let mut contents = [0 as u8; 1048576];
    //let mut handle = file.take(size);

    let no = file.read(&mut contents).unwrap();
    debug!("Read {} bytes from the block file off [{}] size [{}]", no, offset, size );

    stream.write_all(&contents[0..size as usize]).unwrap();
    stream.flush().unwrap();
}

pub fn storage_write(stream: &mut TcpStream, json_data: Value, service: Arc<Mutex<Service>>) {
    let size: usize = json_data["size"].as_u64().unwrap() as usize;
    let file = OpenOptions::new()
        .append(true)
        .open(String::from("./storage.bin"))
        .unwrap();
    let mut fbuf = BufWriter::new(file);

    stream.write_all(String::from("OK").as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut destbuffer = [0 as u8; 2048];
    let mut total = 0 as usize;
    let mut offset = 0 as usize;
    {
        let mut service_instance = service.lock().unwrap();
        loop {
            let dno = stream.read(&mut destbuffer).unwrap();
            total += dno;
            fbuf.write_all(&destbuffer[0..dno]).unwrap();
            fbuf.flush().unwrap();
            if total == size {
                break;
            }
        }

        service_instance.storage.metadata.instance_count += 1;
        offset = service_instance.storage.metadata.current_block_offset as usize;
        service_instance.storage.metadata.current_block_offset += total as u64;
        //println!("index [{}]  Read {} bytes",total, service_instance.faas.metadata.instance_count);
    }
    //println!("{}",total);

    //write to any free block and return the details
    let data = json!({
        "blockno": "no",
        "offset": offset,
        "size": total,
        "index": 0,
       // "c_hash": "hash",
       // "block_hash": "bhash",
    })
    .to_string();
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}
