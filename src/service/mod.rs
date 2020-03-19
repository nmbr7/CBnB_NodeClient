use std::collections::HashMap;

pub mod faas;
pub mod storage;

pub struct Vm {
    pub service_id: String,
}
pub struct Storage {
    pub service_id: String,
}
pub struct Docker {
    pub service_id: String,
}

pub struct Fas {
    pub service_id: String,
    //pub invocations: i32,
    //pub frequency: i32,
    //pub creating_date: i32,
    //pub stat: &'a str, //published or not
}
/*
struct Fas<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
    //pub invocations: i32,
    //pub frequency: i32,
    //pub creating_date: i32,
    //pub stat: &'a str, //published or not
}
*/

pub struct Service {
    pub vms: HashMap<String, Vm>,
    pub storages: HashMap<String, Storage>,
    pub dockerapps: HashMap<String, Docker>,
    pub faas: HashMap<String, Fas>,
}
