#[derive(Debug, PartialEq)]
pub enum ClientMessage {
    Ping,
    Buffer(Vec<u8>),
    Login(Vec<u8>), // 登录
    HeartBeat,      //心跳
    Leave,          //离线
    Invalid,
}

pub fn decode_message(buf: Vec<u8>) -> ClientMessage {
    if buf.is_empty() {
        return ClientMessage::Invalid;
    }
    return ClientMessage::Buffer(buf[2..].to_vec());
}
