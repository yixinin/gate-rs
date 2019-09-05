use  crate::protocol::client_message::{ClientMessage};
use  crate::protocol::center_message::{CenterMessage};
use  crate::protocol::room_message::{RoomMessage};
use ws::Sender;

pub enum ChannelMessage {
    SocketOpen(Sender),
    ClientData(u32, ClientMessage),
    CenterData(CenterMessage),
    RoomData(RoomMessage),
}
