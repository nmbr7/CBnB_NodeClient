
enum MsgType {
     REGISTER,
     UPDATE,
}


struct Message {
    msg_type: MsgType,
    ip: String,
    content: String,

}

impl Message {
    pub fn register() -> String {
        unimplemented!("implement register message format");
    }

    pub fn update() -> String {
        unimplemented!("implement update message format");
    }


}
