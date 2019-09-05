#[derive(Debug, PartialEq)]
pub enum ClientMessage {
    Ping,
    Buffer(Vec<u8>),
    Login(Vec<u8>), // 登录
    Leave,          //离线
    Invalid,
}

pub enum ServerMessage {
    Ping,
    Kick(Vec<i64>),
    Buffer(Vec<i64>, Vec<u8>),
}

pub fn decode_message(buf: Vec<u8>) -> ClientMessage {
    if buf.is_empty() {
        return ClientMessage::Invalid;
    }
    if buf.len() < 2 {
        match buf.get(0) {
            1 => return ClientMessage::Ping,
            _ => return ClientMessage::Leave,
        }
        return ClientMessage::Ping;
    }
    return ClientMessage::Buffer(buf[2..]);
}
