use log::{debug, info, warn};
use serde_json::{json, Value};
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::{Command, Stdio};

pub fn start_qemu() -> u32 {
    let cmd = format!("{} -M pc -m {} -smp 1 -monitor pty -nographic -hda {} -enable-kvm -drive file={},if=virtio,format=raw -net nic -net user,hostfwd=tcp:127.0.0.1:55555-:22,hostfwd=tcp:127.0.0.1:8080-:9090","qemu-system-x86_64",2048,"./xenial.img","./debian-seed.img");
    let args = cmd.split(" ").collect::<Vec<&str>>();

    let a = Command::new(&args[0])
        .args(&args[1..args.len()])
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Error");
    a.id()
}

pub fn new_app(json_data: Value) {
    /*let lang = &json_data["runtime"];
        let filename = &json_data["filename"];
        let fileid = &json_data["fileid"];
    */
    // TODO (Maybe)
    //  SSH to VM using the private key
    //  Generate a public/private key pair for the user PaaS instance
    //  Send the public key and user uuid to the VM
    //  Create a new user of the respective uuid
    //  Return the private key to the PaaS user

    // Update the metadata about the app instance in the `Service Struct`
    // connect to qemu to Deploy new container
    let qemu_ip = String::from("127.0.0.1:9090");
    let mut qstream = TcpStream::connect(qemu_ip).unwrap();
    let msg = json_data.to_string();
    debug!("Deploying new app: \n{}", msg);

    qstream.write_all(msg.as_bytes()).unwrap();
    qstream.flush().unwrap();
}
