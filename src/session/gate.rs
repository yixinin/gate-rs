use crate::protocol::center_message::CenterMessage;
use crate::protocol::client_message::ClientMessage;
use crate::protocol::room_message::RoomMessage;
use crate::session::center_state::CenterState;
use crate::session::channel_message::ChannelMessage::*;
use crate::session::client_state::ClientState;
use crate::session::room_state::RoomState;
use crate::session::ChannelMessage;
use crossbeam_channel::Receiver;
use crossbeam_channel::*;
use crate::config::AppConfig;

use crate::protocol::packet;
// use libc::recv;

use ws::Sender;

pub struct Gate {
    pub client_state: ClientState,
    pub center_state: CenterState,
    pub room_state: RoomState,
    pub config : AppConfig,
}
impl Gate {
    pub fn listen(&mut self, clients_channel: Receiver<ChannelMessage>) {
        loop {
            let message = select! {
                recv(clients_channel) -> peer => peer.unwrap()
            };

            //  let message = recv(clients_channel);
            match message {
                ChannelMessage::ClientData(cid, data) => self.on_message(cid, &data),
                ChannelMessage::CenterData(data) => match data {
                    CenterMessage::Buffer(userids, buf) => self.send_message(userids, buf),
                    CenterMessage::Kick(userid) => self.kick(userid),
                    _ => (),
                },
                ChannelMessage::RoomData(data) => match data {
                    RoomMessage::Buffer(userids, buf) => self.send_message(userids, buf),
                    _ => (),
                },
                _ => (),
            };
        }
    }

    pub fn on_open(&mut self, sender: Sender) {}

    pub fn on_message(&self, cid: u32, message: &ClientMessage) {
        match message {
            ClientMessage::HeartBeat(buf) => self.heart_beat(cid),
            ClientMessage::Leave => self.on_logout(cid),
            ClientMessage::Login(buf) => self.on_login(cid, buf),
            _=>(),
        }
    }

    pub fn heart_beat(&self, cid: u32) {
        if let Some(sender) = self.client_state.connid_socket.get(&cid) {
            sender.send(packet::bytes_to_msg(vec![1, 2]));
        }
    }

    pub fn kick(&mut self, userid: Vec<i64>) {}
 
    pub fn on_login(&self, cid: u32, buf : &Vec<u8>) {}

    pub fn on_logout(&self, cid: u32) {}

    pub fn send_message(&self, userid: Vec<i64>, message: Vec<u8>) {}
}
