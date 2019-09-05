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

const METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE: ::grpcio::Method<super::center::Gate2CenterRequest, super::center::Gate2CenterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Center/Gate2CenterMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct Gate2CenterClient {
    client: ::grpcio::Client,
}

impl Gate2CenterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Gate2CenterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn gate2_center_message_opt(&self, req: &super::center::Gate2CenterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::Gate2CenterResponse> {
        self.client.unary_call(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, req, opt)
    }

    pub fn gate2_center_message(&self, req: &super::center::Gate2CenterRequest) -> ::grpcio::Result<super::center::Gate2CenterResponse> {
        self.gate2_center_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn gate2_center_message_async_opt(&self, req: &super::center::Gate2CenterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Gate2CenterResponse>> {
        self.client.unary_call_async(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, req, opt)
    }

    pub fn gate2_center_message_async(&self, req: &super::center::Gate2CenterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Gate2CenterResponse>> {
        self.gate2_center_message_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Gate2Center {
    fn gate2_center_message(&mut self, ctx: ::grpcio::RpcContext, req: super::center::Gate2CenterRequest, sink: ::grpcio::UnarySink<super::center::Gate2CenterResponse>);
}

pub fn create_gate2_center<S: Gate2Center + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, move |ctx, req, resp| {
        instance.gate2_center_message(ctx, req, resp)
    });
    builder.build()
}

const METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE: ::grpcio::Method<super::center::Center2GateRequest, super::center::Center2GateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Center2Gate/Center2GateMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct Center2GateClient {
    client: ::grpcio::Client,
}

impl Center2GateClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Center2GateClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn center2_gate_message_opt(&self, req: &super::center::Center2GateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::Center2GateResponse> {
        self.client.unary_call(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, req, opt)
    }

    pub fn center2_gate_message(&self, req: &super::center::Center2GateRequest) -> ::grpcio::Result<super::center::Center2GateResponse> {
        self.center2_gate_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn center2_gate_message_async_opt(&self, req: &super::center::Center2GateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Center2GateResponse>> {
        self.client.unary_call_async(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, req, opt)
    }

    pub fn center2_gate_message_async(&self, req: &super::center::Center2GateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Center2GateResponse>> {
        self.center2_gate_message_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Center2Gate {
    fn center2_gate_message(&mut self, ctx: ::grpcio::RpcContext, req: super::center::Center2GateRequest, sink: ::grpcio::UnarySink<super::center::Center2GateResponse>);
}

pub fn create_center2_gate<S: Center2Gate + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, move |ctx, req, resp| {
        instance.center2_gate_message(ctx, req, resp)
    });
    builder.build()
}
