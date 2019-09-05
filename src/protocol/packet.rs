use std::ops::Shr;
use ws::Message;

pub fn bytes_to_msg(buf: Vec<u8>) -> Message {
    Message::Binary(buf)
}
