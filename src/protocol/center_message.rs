pub enum CenterMessage {
    Ping,
    Kick(Vec<i64>),
    Buffer(Vec<i64>, Vec<u8>),
}
