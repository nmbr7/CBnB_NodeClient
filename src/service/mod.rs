use std::collections::HashMap;

pub mod faas;
pub mod storage;
pub mod paas;


/*
usee chrono::pr;
let  utc = Utc::now().timestamp().to_string().to_owned();
let st = Utc.datetime_from_str(utc.as_str(), "%s");
println!("{}",st.unwrap());

 */

pub struct Storage {
    pub block: i32,
    pub created_on: String,
    pub offsets: Vec<i32>,
    pub frequency: i32,
}

pub struct Paas {
    pub created_on: String,

}

pub struct Fas {
    pub invocations: i32,
    pub frequency: i32,
    pub created_on: String,
    pub status1: String, //published or not
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
pub struct MetaData {
    instance_count : i32,
    last_updated: (String,String),
}

pub struct ServiceData<T> {
    metadata: MetaData,
    instances: HashMap<String,T>,
}

pub struct Service {
    pub paas: ServiceData<Paas>, 
    pub storage: ServiceData<Storage>,
    pub faas: ServiceData<Fas>,
}
