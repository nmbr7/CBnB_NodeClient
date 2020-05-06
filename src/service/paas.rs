use dotenv::dotenv;
use log::{debug, info, warn};
use serde_json::{json, Value};
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::{Command, Stdio};

pub fn start_qemu() -> u32 {
    dotenv().ok();
    let run_mode = env::var("RUN_MODE").expect("RUN_MODE not set");
    let qfolder = match run_mode.as_str() {
        "TEST" => String::from("/qemufolder/"),
        "DEV" => {
            String::from("/home/number7/Desktop/PROJECTS/CloudBnB_Root/DockerInfra/qemufolder/")
        }
        _ => panic!("Run mode not set"),
    };

    let cmd = format!("qemu-system-x86_64 -M pc -m 2048 -smp 4 -monitor pty -nographic -hda {qf}xenial_base.img -drive file={qf}debian-seed.img,if=virtio,format=raw -enable-kvm -netdev user,hostname=be5d9d96fd08,hostfwd=tcp::55555-:22,hostfwd=tcp:0.0.0.0:7070-:8080,id=net -device virtio-net-pci,netdev=net -vnc :0 -serial stdio",qf=qfolder);
    let args = cmd.split(" ").collect::<Vec<&str>>();
    println!("{:?}", cmd);
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
    let qemu_ip = String::from("127.0.0.1:7070");
    let mut qstream = TcpStream::connect(qemu_ip).unwrap();
    let msg = json_data.to_string();
    debug!("Deploying new app: \n{}", msg);

    qstream.write_all(msg.as_bytes()).unwrap();
    qstream.flush().unwrap();
}
