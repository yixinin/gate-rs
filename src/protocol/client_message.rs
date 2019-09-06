#[derive(Debug, PartialEq)]
pub enum ClientMessage {
    Buffer(Vec<u8>),    //数据包
    Login(Vec<u8>),     //登录
    HeartBeat(Vec<u8>), //心跳
    Logout(Vec<u8>),     //离线
    Leave, //断线
    Invalid,
}

pub fn decode_message(buf: Vec<u8>) -> ClientMessage {
    if buf.is_empty() {
        return ClientMessage::Invalid;
    }
    return ClientMessage::Buffer(buf[2..].to_vec());
}
