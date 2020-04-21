use std::process::{Command, Stdio};

pub fn start_qemu() {
    let cmd = format!("{} -M pc -m {} -smp 1 -monitor pty -nographic -hda {} -drive file={},if=virtio,format=raw -net nic -net user,hostfwd=tcp:127.0.0.1:55555-:22,hostfwd=tcp:127.0.0.1:8080-:9090","qemu-system-x86_64",2048,"./xenial.img","./debian-seed.img");
    let args = cmd.split(" ").collect::<Vec<&str>>();

    let a = Command::new(&args[0])
        .args(&args[1..args.len()])
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Error");
}
