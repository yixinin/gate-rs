use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use grpcio::{ChannelBuilder, EnvBuilder};
use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use grpcio::{Server,Client};

use crate::protocol::center_grpc::create_center2_gate;
use crate::protocol::center_grpc::{Center2Gate,Gate2CenterClient};
use crate::protocol::center::{Center2GateRequest,Center2GateResponse};
use crate::protocol::center::{Gate2CenterRequest,Gate2CenterResponse};

#[derive(Clone)]
struct Center2GateService;

impl Center2Gate for Center2GateService{
     fn center2_gate_message(&mut self, ctx: ::grpcio::RpcContext, req: Center2GateRequest, sink: UnarySink<Center2GateResponse>){

     }
}

pub struct CenterState{
    pub server : Server, 
    pub client : Gate2CenterClient,
} 


impl CenterState{

    pub fn new() -> CenterState{
         let env = Arc::new(Environment::new(1));
        let service = create_center2_gate(Center2GateService);
        let s = ServerBuilder::new(env.clone())
            .register_service(service)
            .bind("0.0.0.0", 40041)
            .build()
            .unwrap();
       

        let ch = ChannelBuilder::new(env).connect("localhost:50051");
        let c = Gate2CenterClient::new(ch);
        
        CenterState{
            client:c,
            server:s,
        }
    }
    pub fn linsten(&mut self){  
        self.server.start();
        for &(ref host,port ) in self.server.bind_addrs(){
            println!("gate listen center msg on {}:{}", host,port);
        }
        let (tx,rx) = oneshot::channel();
        thread::spawn(move ||{
            println!("{}", "Press Enter to exit...");
            let _=io::stdin().read(&mut [0]).unwrap();
            tx.send(())
        });
        let _=rx.wait();
        let _=self.server.shutdown();
    }

    pub fn send(&mut self){
       let mut req = Gate2CenterRequest::default();
        
       let reply = self.client.gate2_center_message(&req).expect("rpc");
    }
}  