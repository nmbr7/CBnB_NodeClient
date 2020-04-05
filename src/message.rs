use crate::sys_stat;
use crate::sys_stat::GetStat;
use serde::{Deserialize, Serialize};
use serde_json::Result;

////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Storage,
    Faas,
    Paas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceMsgType {
    //  UPDATE_SYSTAT,
    SERVICEINIT,
    SERVICEUPDATE,
    //SERVICESTART,
    //SERVICESTOP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMessage {
    pub uuid: String,
    pub msg_type: ServiceMsgType,
    pub service_type: ServiceType,
    pub content: String,
}

/////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgType {
    REGISTER,
    UPDATE_SYSTAT,
}

#[derive(Debug, Serialize)]
pub struct NodeMessage {
    uuid: String,
    msg_type: MsgType,
    content: String, //sys_stat::Resources,
}

#[derive(Debug, Serialize)]
pub enum Message {
    Node(NodeMessage),
}

impl NodeMessage {
    fn new(msg_type: MsgType, content: String) -> Self {
        Self {
            uuid: "Node_uuid".to_string(),
            msg_type,
            content,
        }
    }
    pub fn register(stat: sys_stat::Resources) -> String {
        let s = serde_json::to_string(&stat).unwrap();
        let msg = Message::Node(NodeMessage::new(MsgType::REGISTER, s));
        serde_json::to_string(&msg).unwrap()
    }

    pub fn update(stat: sys_stat::StatUpdate) -> String {
        let s = serde_json::to_string(&stat).unwrap();
        let msg = Message::Node(NodeMessage::new(MsgType::UPDATE_SYSTAT, s));
        serde_json::to_string(&msg).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let stat = sys_stat::Resources::new();
        //  println!("{:?}",Message::new(MsgType::REGISTER,stat))
        println!("{}", Message::<sys_stat::Resources>::register(stat))
    }
}
