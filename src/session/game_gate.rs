use crossbeam_channel::Receiver;
use session::channel_message::ChannelMessage::*;
use session::client_state::ClientState;
use session::ChannelMessage;

pub struct GameGate {
    pub client_state: ClientState,
    pub center_state: CenterState,
    pub game_state: GameState,
}

impl GameGate {
    pub fn listen(&mut self, clients_channel: Reciver<ChannelMessage>) {
        loop {
            let message = select! {
                recv(clients_channel,received) =>received.unwarp()
            };
            match message {
                ClientData(data)=>,
            }
        }
    }

    pub fn on_open(&self, sender:Sender){

    }

    pub fn on_message(&self,cid :u32,message :&ClientMessage){

    }

    pub fn on_login(&self,cid : u32,message:&ClientMessage){

    }

    pub fn on_logout(&self,cid:u32,message :&ClientMessage){

    }


    pub fn send_message(&self,cid u32,message:Vec<u8>){

    }
}
