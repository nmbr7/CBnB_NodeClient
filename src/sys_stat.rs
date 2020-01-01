// This module contains all the strctures and function which checks the system 
// status periodically
extern crate sysinfo;
use sysinfo::{System, Processor, SystemExt, ProcessorExt, NetworkExt};

#[derive(Debug)]
struct Resources {
    ram: u64,
    //cpu_frequency: f32,
    //core_count: u8,
    //net_speed_up: u64,
    //net_speed_down: f32,
    //disk_storage: f32,
    //gpu: bool,
}

impl Resources{
    fn available() -> Self {
        let mut sys = System::new();
        let mut processor = Processor::new();
        sys.refresh_all();
        let ram = sys.get_total_memory();
        Self{
            ram,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn run() {
        println!("{:?}",Resources::available());

    }

}
