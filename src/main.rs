extern crate grpcio;
extern crate ws;
extern crate futures;
extern crate libc; 
extern crate protobuf;

extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel as channel;
use session::gate::Gate;
use std::thread;

use ws::Builder;
use ws::Settings;

use session::ChannelMessage;

pub mod protocol;
pub mod session;
 
fn main() {
    let (sender, receiver) = channel::unbounded::<ChannelMessage>();

    // thread::spawn(move || {
    //     Gate {
    //         room_number: 0,
    //         client_state: ClientState::default(), 
    //     }
    //     .listen(receiver);
    // });
}
 