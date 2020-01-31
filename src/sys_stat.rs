// This module contains all the strctures and function which checks the system
// status periodically
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread::sleep;
use std::time::{Duration, Instant};
use uuid::Uuid;

//###################
// Macros

#[macro_export]
macro_rules! readfieldws {
    ($a:expr,$b:expr,$c:expr) => {
        $a.clone()
            .into_iter()
            .nth($b)
            .unwrap()
            .split_whitespace()
            .nth($c)
            .unwrap()
            .parse()
            .unwrap();
    };
}
macro_rules! readfieldwcs {
    ($a:expr,$b:expr,$c:expr) => {
        $a.clone()[$b]
            .split(':')
            .map(|l| l.to_string())
            .collect::<Vec<String>>()[$c]
            .trim()
            .to_string();
    };
}

pub trait GetStat {
    fn new() -> Self;
    // Do error handling
    fn update(&mut self) -> ();
}

fn fetch(path: String) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    lines.map(|l| l.expect("Parse Fail")).collect()
}
//###########################################
#[derive(Debug, Clone, Serialize)]
pub struct StatUpdate {
    uuid: String,
    cpu_usage: String,
    mem_usage: (String, String),
    mem_free: String,
    mem_available: String,
    net: NetInfo,
    uptime: String,
    //ram: u64,
    //cpu_frequency: f32,
    //core_count: u9,
    //net_speed_up: u64,
    //net_speed_down: f32,
    //disk_storage: f32,
    //gpu: bool,
}

//############################################
//#[derive(Debug)]
#[derive(Debug, Clone, Serialize)]
pub struct Resources {
    uuid: String,
    cpu: CpuInfo,
    mem: MemInfo,
    net: NetInfo,
    uptime: String,
    //cpu_frequency: f32,
    //core_count: u9,
    //disk_storage: f32,
    //gpu: bool,
}

impl Resources {
    pub fn update_stat(&mut self) -> StatUpdate {
        self.update();
        let a = fetch(format!("/proc/uptime"));
        let uptime: String = readfieldws!(a, 0, 0);
        StatUpdate {
            uuid: self.uuid.clone(),
            cpu_usage: self.cpu.usage.clone(),
            mem_usage: self.mem.usage.clone(),
            mem_free: self.mem.free.clone(),
            mem_available: self.mem.available.clone(),
            net: self.net.clone(),
            uptime: uptime,
        }
    }
}
impl GetStat for Resources {
    fn new() -> Self {
        let core_uuid = Uuid::new_v4().to_string();
        let a = fetch(format!("/proc/uptime"));
        let uptime: String = readfieldws!(a, 0, 0);
        Self {
            uuid: core_uuid,
            cpu: CpuInfo::new(),
            mem: MemInfo::new(),
            net: NetInfo::new(),
            uptime: uptime,
        }
    }

    fn update(&mut self) -> () {
        self.cpu.update();
        self.mem.update();
        self.net.update();
    }
}
/*
//############################################
// Struct to store the cpu vendor and model details
#[derive(Debug, Clone, Serialize)]
struct CpuModel {
    vendor: String,
    cpu_family: String,
    model: String,
    model_name: String,
}
*/

// Struct to store the cpu details of the system - output similar to lscpu
#[derive(Debug, Clone, Serialize)]
struct CpuInfo {
    //arch: String,
    //op_model: String,
    //byte_order: String,
    //cpus: u8,
    model: String,
    cputime: (i64, i64),
    //virtualization: String,
    usage: String,
}

impl CpuInfo {
    fn time() -> (i64, i64) {
        let cpu_stat = fetch(format!("/proc/stat"));

        let total: i64 = cpu_stat
            .clone()
            .into_iter()
            .nth(0)
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|p| p.parse::<i64>().unwrap())
            .sum();
        let idel: i64 = readfieldws!(cpu_stat, 0, 4);
        (total, idel)
    }
}
impl GetStat for CpuInfo {
    fn new() -> Self {
        let cpu_info = fetch(format!("/proc/cpuinfo"));
        let cpumodel = vec![
            readfieldwcs!(cpu_info, 1, 1),
            readfieldwcs!(cpu_info, 2, 1),
            readfieldwcs!(cpu_info, 3, 1),
            readfieldwcs!(cpu_info, 4, 1),
        ]
        .join(",");
        let (oldtotal_time, oldidel_time) = Self::time();
        sleep(Duration::from_secs(2));
        let (total_time, idel_time) = Self::time();
        let c_usage = 100.0
            * (1.0 - ((idel_time - oldidel_time) as f64 / (total_time - oldtotal_time) as f64));

        Self {
            model: cpumodel,
            cputime: (total_time, idel_time),
            usage: format!("{:.2}", c_usage),
        }
    }

    fn update(&mut self) -> () {
        let (oldtotal_time, oldidel_time) = self.cputime;
        let (total_time, idel_time) = Self::time();
        let c_usage = 100.0
            * (1.0 - ((idel_time - oldidel_time) as f64 / (total_time - oldtotal_time) as f64));

        self.cputime = (total_time, idel_time);
        self.usage = format!("{:.2}", c_usage);
    }
}

//############################################
#[derive(Clone, Debug, Serialize)]
struct NetInfo {
    //interfaces: String,
    current_interface: String,
    speed: (String, String),
    //ip: String,
}

impl NetInfo {
    fn bandwidth(c_iface: String) -> (String, String) {
        // To calculate  bandwidth by downloading a file
        /*
        let mut stream = TcpStream::connect("212.183.159.230:80").unwrap();
        let (testhost, testpath) = (String::from("212.183.159.230"), String::from("/5MB.zip"));
        //let mut v:Vec<u8> = Vec::new();
        let mut avg = 0 as f64;
        let mut count = 0 as f64;
        let header = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", testpath, testhost);
        stream.write_all(header.as_bytes()).unwrap();
        */
        fn byt(iface: String) -> (i64, i64) {
            let f = fetch(format!("/proc/net/dev"));
            let bytes = f
                .into_iter()
                .filter(|line| {
                    line.split_whitespace()
                        .nth(0)
                        .map(|l| l == format!("{}:", iface))
                        .unwrap_or(false)
                })
                .last()
                .unwrap();
            let rxs: i64 = bytes.split_whitespace().nth(1).unwrap().parse().unwrap();
            let txs: i64 = bytes.split_whitespace().nth(10).unwrap().parse().unwrap();
            (rxs, txs)
        }

        let tnow = Instant::now();
        let (r1, t1) = byt(c_iface.clone());
        sleep(Duration::from_secs(1));
        let (r2, t2) = byt(c_iface.clone());
        let sec = tnow.elapsed().as_secs_f64();
        let rx = format!(
            "{:.5}",
            ((((r2 - r1) as f64) / (1024 * 1024) as f64) / sec as f64)
        );
        let tx = format!(
            "{:.5}",
            ((((t2 - t1) as f64) / (1024 * 1024) as f64) / sec as f64)
        );
        (rx, tx)

        // To calculate  bandwidth by downloading a file ,Buffering
        /*loop {
            let mut buf = [0; 512];
            let now = Instant::now();
            //let a=stream.read_to_end(&mut v).unwrap();
            let a = stream.read(&mut buf).unwrap();
            let sec = now.elapsed().as_secs_f64();
            if a == 0 {
                break;
            }
            count += sec;
            avg = avg + (a as f64 / (1024 * 1024) as f64);
            //println!("Speed : {:?} MBps\n Total Time : {} ",avg/count as f64,sec);
        }
        let speed = format!("Speed: {:?} MBps Time: {} ",avg / count as f64,sec);
        */
    }
}

impl GetStat for NetInfo {
    fn new() -> Self {
        let net_route = fetch(format!("/proc/net/route"));

        let c_iface = net_route
            .into_iter()
            .filter(|line| {
                line.split_whitespace()
                    .nth(2)
                    .map(|l| l != "00000000")
                    .unwrap_or(false)
            })
            .last()
            .unwrap()
            .split_whitespace()
            .nth(0)
            .unwrap()
            .to_owned();

        //TODO add check for case when no iface is up
        //println!("*********{}*********",c_iface);
        let speed = Self::bandwidth(c_iface.clone());
        Self {
            //interfaces: c_iface,
            current_interface: c_iface.clone(),
            speed: speed,
        }
    }

    fn update(&mut self) -> () {
        let net_route = fetch(format!("/proc/net/route"));

        let c_iface = net_route
            .into_iter()
            .filter(|line| {
                line.split_whitespace()
                    .nth(2)
                    .map(|l| l != "00000000")
                    .unwrap_or(false)
            })
            .last()
            .unwrap()
            .split_whitespace()
            .nth(0)
            .unwrap()
            .to_owned();

        if self.current_interface != c_iface {
            self.current_interface = c_iface.clone();
        }
        let speed = Self::bandwidth(c_iface.clone());
        self.speed = speed;
    }
}

//############################################
#[derive(Debug, Clone, Serialize)]
struct MemInfo {
    usage: (String, String),
    total: String,
    //used: String,
    free: String,
    available: String,
    //swap: String,
}

impl GetStat for MemInfo {
    fn new() -> Self {
        let meminfo = fetch(format!("/proc/meminfo"));

        let total_mem: f64 = readfieldws!(meminfo, 0, 1);
        let free_mem: f64 = readfieldws!(meminfo, 1, 1);
        let available_mem: f64 = readfieldws!(meminfo, 2, 1);
        let buffer_mem: f64 = readfieldws!(meminfo, 3, 1);
        let cache_mem: f64 = readfieldws!(meminfo, 4, 1);
        let sreclaim_mem: f64 = readfieldws!(meminfo, 22, 1);
        let shmem_mem: f64 = readfieldws!(meminfo, 20, 1);
        let total_mem_usage = 100.0 * ((total_mem - free_mem) / total_mem);
        let mem_usage_ncb = 100.0
            * ((total_mem - free_mem - (cache_mem + sreclaim_mem - shmem_mem + buffer_mem))
                / total_mem);
        Self {
            usage: (
                format!("{:.3}", total_mem_usage),
                format!("{:.3}", mem_usage_ncb),
            ),
            total: format!("{:.3}", (total_mem / (1024 * 1024) as f64)),
            free: format!("{:.3}", (free_mem / (1024 * 1024) as f64)),
            available: format!("{:.3}", (available_mem / (1024 * 1024) as f64)),
        }
    }

    fn update(&mut self) -> () {
        let meminfo = fetch(format!("/proc/meminfo"));
        let total_mem: f64 = readfieldws!(meminfo, 0, 1);
        let free_mem: f64 = readfieldws!(meminfo, 1, 1);
        let available_mem: f64 = readfieldws!(meminfo, 2, 1);
        let buffer_mem: f64 = readfieldws!(meminfo, 3, 1);
        let cache_mem: f64 = readfieldws!(meminfo, 4, 1);
        let sreclaim_mem: f64 = readfieldws!(meminfo, 22, 1);
        let shmem_mem: f64 = readfieldws!(meminfo, 20, 1);
        let total_mem_usage = 100.0 * ((total_mem - free_mem) / total_mem);
        let mem_usage_ncb = 100.0
            * ((total_mem - free_mem - (cache_mem + sreclaim_mem - shmem_mem + buffer_mem))
                / total_mem);
        self.usage = (
            format!("{:.3}", total_mem_usage),
            format!("{:.3}", mem_usage_ncb),
        );
        self.free = format!("{:.3}", (free_mem / (1024 * 1024) as f64));
        self.available = format!("{:.3}", (available_mem / (1024 * 1024) as f64));
    }
}

//############################################
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        //        let mut res = Resources::new();
        //      println!("{:?}", res);
        //      sleep(Duration::from_secs(5));
        //      let stat_update = res.update();
        //      println!("{:?}", stat_update);
        let a = fetch(format!("/proc/uptime"));
        let v: String = readfieldws!(a, 0, 0);
        println!("{}", v);
    }
}
