use protocol::client_message::{ClientMessage, ServerMessage};
use ws::Sender;

pub enum ChannelMessage {
    SocketOpen(Sender),
    ClientData(u32, ClientMessage),
    ServerData(i64, ServerMessage),
}
