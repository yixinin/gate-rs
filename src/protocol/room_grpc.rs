// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GATE2_ROOM_GATE2_ROOM_MESSAGE: ::grpcio::Method<super::room::Gate2RoomReq, super::room::Gate2RoomAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Room/Gate2RoomMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct Gate2RoomClient {
    client: ::grpcio::Client,
}

impl Gate2RoomClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Gate2RoomClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn gate2_room_message_opt(&self, req: &super::room::Gate2RoomReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::room::Gate2RoomAck> {
        self.client.unary_call(&METHOD_GATE2_ROOM_GATE2_ROOM_MESSAGE, req, opt)
    }

    pub fn gate2_room_message(&self, req: &super::room::Gate2RoomReq) -> ::grpcio::Result<super::room::Gate2RoomAck> {
        self.gate2_room_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn gate2_room_message_async_opt(&self, req: &super::room::Gate2RoomReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::room::Gate2RoomAck>> {
        self.client.unary_call_async(&METHOD_GATE2_ROOM_GATE2_ROOM_MESSAGE, req, opt)
    }

    pub fn gate2_room_message_async(&self, req: &super::room::Gate2RoomReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::room::Gate2RoomAck>> {
        self.gate2_room_message_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Gate2Room {
    fn gate2_room_message(&mut self, ctx: ::grpcio::RpcContext, req: super::room::Gate2RoomReq, sink: ::grpcio::UnarySink<super::room::Gate2RoomAck>);
}

pub fn create_gate2_room<S: Gate2Room + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_ROOM_GATE2_ROOM_MESSAGE, move |ctx, req, resp| {
        instance.gate2_room_message(ctx, req, resp)
    });
    builder.build()
}

const METHOD_ROOM2_GATE_ROOM2_GATE_MESSAGE: ::grpcio::Method<super::room::Room2GateReq, super::room::Room2GateAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Room2Gate/Room2GateMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct Room2GateClient {
    client: ::grpcio::Client,
}

impl Room2GateClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Room2GateClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn room2_gate_message_opt(&self, req: &super::room::Room2GateReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::room::Room2GateAck> {
        self.client.unary_call(&METHOD_ROOM2_GATE_ROOM2_GATE_MESSAGE, req, opt)
    }

    pub fn room2_gate_message(&self, req: &super::room::Room2GateReq) -> ::grpcio::Result<super::room::Room2GateAck> {
        self.room2_gate_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn room2_gate_message_async_opt(&self, req: &super::room::Room2GateReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::room::Room2GateAck>> {
        self.client.unary_call_async(&METHOD_ROOM2_GATE_ROOM2_GATE_MESSAGE, req, opt)
    }

    pub fn room2_gate_message_async(&self, req: &super::room::Room2GateReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::room::Room2GateAck>> {
        self.room2_gate_message_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Room2Gate {
    fn room2_gate_message(&mut self, ctx: ::grpcio::RpcContext, req: super::room::Room2GateReq, sink: ::grpcio::UnarySink<super::room::Room2GateAck>);
}

pub fn create_room2_gate<S: Room2Gate + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROOM2_GATE_ROOM2_GATE_MESSAGE, move |ctx, req, resp| {
        instance.room2_gate_message(ctx, req, resp)
    });
    builder.build()
}
