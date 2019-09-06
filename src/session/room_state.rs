use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{ChannelBuilder, EnvBuilder};
use grpcio::{Client, Server};
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use crate::protocol::room::{Gate2RoomAck, Gate2RoomReq};
use crate::protocol::room::{Room2GateAck, Room2GateReq};
use crate::protocol::room_grpc::create_room2_gate;
use crate::protocol::room_grpc::{Gate2RoomClient, Room2Gate};

#[derive(Clone)]
struct Room2GateService;

impl Room2Gate for Room2GateService {
    fn room2_gate_message(
        &mut self,
        ctx: RpcContext,
        req: Room2GateReq,
        sink: UnarySink<Room2GateAck>,
    ) {

    }
}

pub struct RoomState {
    pub server: Server,
    pub client: Gate2RoomClient,
}

impl RoomState {
    pub fn new() -> RoomState {
        let env = Arc::new(Environment::new(1));
        let service = create_room2_gate(Room2GateService);
        let s = ServerBuilder::new(env.clone())
            .register_service(service)
            .bind("0.0.0.0", 40041)
            .build()
            .unwrap();

        let ch = ChannelBuilder::new(env).connect("localhost:50051");
        let c = Gate2RoomClient::new(ch);

        RoomState {
            client: c,
            server: s,
        }
    }
    pub fn linsten(&mut self) {
        self.server.start();
        for &(ref host, port) in self.server.bind_addrs() {
            println!("gate listen room msg on {}:{}", host, port);
        }
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            println!("{}", "Press Enter to exit...");
            let _ = io::stdin().read(&mut [0]).unwrap();
            tx.send(())
        });
        let _ = rx.wait();
        let _ = self.server.shutdown();
    }

    pub fn send(&mut self) {
        let mut req = Gate2RoomReq::default();

        let reply = self.client.gate2_room_message(&req).expect("rpc");
    }
}
