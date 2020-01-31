use crate::sys_stat;
use crate::sys_stat::GetStat;
use serde::Serialize;
use serde_json::Result;

#[derive(Debug, Serialize)]
enum MsgType {
    REGISTER,
    UPDATE_SYSTAT,
}

#[derive(Debug, Serialize)]
pub struct Message<T> {
    msg_type: MsgType,
    content: T, //sys_stat::Resources,
}

impl<T> Message<T> {
    fn new(msg_type: MsgType, content: T) -> Self {
        Self { msg_type, content }
    }
    pub fn register(stat: sys_stat::Resources) -> String {
        let s = serde_json::to_string(&stat).unwrap();
        let msg = Message::new(MsgType::REGISTER, s);
        serde_json::to_string(&msg).unwrap()
    }

    pub fn update(stat: sys_stat::StatUpdate) -> String {
        let s = serde_json::to_string(&stat).unwrap();
        let msg = Message::new(MsgType::UPDATE_SYSTAT, s);
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
