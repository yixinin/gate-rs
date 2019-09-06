
use crate::session::channel_message::ChannelMessage;
use crate::session::channel_message::ChannelMessage::*;
use ws::CloseCode;
use ws::Handler;
use ws::Handshake;
use ws::Message;
use ws::Sender;

use crate::protocol::client_message::ClientMessage::*;
use crate::protocol::client_message::decode_message;
use crate::protocol::packet::bytes_to_msg;
use crate::session::client_state::ClientState;

pub struct Connection {
    pub ws: Sender,
    pub room_channel: crossbeam_channel::Sender<ChannelMessage>,
}

impl Handler for Connection {
    fn on_open(&mut self, _shake: Handshake) -> ws::Result<()> {
        self.room_channel.send(SocketOpen(self.ws.clone()));
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let connection_id = self.ws.connection_id();
        let command = decode_message(msg.into_data());
        match command {
            Ping => {
                self.ws.send(bytes_to_msg(vec![111])).ok();
            }
            _ => {
                self.room_channel.send(ClientData(connection_id, command));
            }
        };
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        if code.ne(&ClientState::GAME_CLOSE_CODE) {
            let connection_id = self.ws.connection_id();
            self.room_channel
                .send(ClientData(connection_id, Leave));
        }
    }
}
