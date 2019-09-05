use std::collections::HashMap;
use ws::CloseCode;
use ws::Sender;

pub struct ClientState {
    pub connid_socket: HashMap<u32, Sender>,
    pub userid_connid: HashMap<i64, u32>,
    pub connid_userid: HashMap<u32, i64>,
}

impl Default for ClientState {
    fn default() -> ClientState {
        ClientState {
            connid: HashMap::new(),
            userid_connid: HashMap::new(),
            connid_userid: HashMap::new(),
        }
    }
}

impl ClientState {
    pub const GAME_CLOSE_CODE: CloseCode = CloseCode::Other(4000);

    pub fn add_client(&mut self, userid, sender: Sender) -> Result<(), (Sender, &str)> {
        if self.connid_socket.contains_key(&sender.connection_id()) {
            return Err((sender, "Duplicate connection id"));
        }

        if self.userid_connid.contains_key(userid){
              return Err((sender, "Duplicate connection id"));
        } 
        self.connid_socket.insert(sender.connection_id(), sender);
        self.userid_connid.insert(userid, sender.connection_id());
        self.connid_userid.insert(sender.connection_id(), userid);
        Ok()
    }

    pub fn disconnect(&mut self,userid i64){
        if let Some(sender) = self.get_sender_by_userid(userid){
            sender.close(GAME_CLOSE_CODE);
        }
        if let Some(connid) = self.userid_connid.get(&userid) {
           self.connid_socket.remove(connid);
           self.connid_userid.remove(connid);
        }
        self.userid_connid.remove(userid);
    }

    pub fn get_sender_by_userid(&self,userid:i64) -> Option<&Sender>{
        match self.userid_connid.get(&userid){
            Some(connid) => self.connid_socket.get(connid),
            None=>Option::None
        }
    }
}
