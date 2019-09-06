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

const METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE: ::grpcio::Method<super::center::Gate2CenterReq, super::center::Gate2CenterAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Center/Gate2CenterMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATE2_CENTER_LOGIN: ::grpcio::Method<super::center::LoginReq, super::center::LoginAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Center/Login",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATE2_CENTER_LOGOUT: ::grpcio::Method<super::center::LoginAck, super::center::LogoutAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Center/Logout",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATE2_CENTER_HEART_BEAT: ::grpcio::Method<super::center::HeartBeatReq, super::center::HeartBeatAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Gate2Center/HeartBeat",
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

    pub fn gate2_center_message_opt(&self, req: &super::center::Gate2CenterReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::Gate2CenterAck> {
        self.client.unary_call(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, req, opt)
    }

    pub fn gate2_center_message(&self, req: &super::center::Gate2CenterReq) -> ::grpcio::Result<super::center::Gate2CenterAck> {
        self.gate2_center_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn gate2_center_message_async_opt(&self, req: &super::center::Gate2CenterReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Gate2CenterAck>> {
        self.client.unary_call_async(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, req, opt)
    }

    pub fn gate2_center_message_async(&self, req: &super::center::Gate2CenterReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Gate2CenterAck>> {
        self.gate2_center_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn login_opt(&self, req: &super::center::LoginReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::LoginAck> {
        self.client.unary_call(&METHOD_GATE2_CENTER_LOGIN, req, opt)
    }

    pub fn login(&self, req: &super::center::LoginReq) -> ::grpcio::Result<super::center::LoginAck> {
        self.login_opt(req, ::grpcio::CallOption::default())
    }

    pub fn login_async_opt(&self, req: &super::center::LoginReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::LoginAck>> {
        self.client.unary_call_async(&METHOD_GATE2_CENTER_LOGIN, req, opt)
    }

    pub fn login_async(&self, req: &super::center::LoginReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::LoginAck>> {
        self.login_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn logout_opt(&self, req: &super::center::LoginAck, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::LogoutAck> {
        self.client.unary_call(&METHOD_GATE2_CENTER_LOGOUT, req, opt)
    }

    pub fn logout(&self, req: &super::center::LoginAck) -> ::grpcio::Result<super::center::LogoutAck> {
        self.logout_opt(req, ::grpcio::CallOption::default())
    }

    pub fn logout_async_opt(&self, req: &super::center::LoginAck, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::LogoutAck>> {
        self.client.unary_call_async(&METHOD_GATE2_CENTER_LOGOUT, req, opt)
    }

    pub fn logout_async(&self, req: &super::center::LoginAck) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::LogoutAck>> {
        self.logout_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heart_beat_opt(&self, req: &super::center::HeartBeatReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::HeartBeatAck> {
        self.client.unary_call(&METHOD_GATE2_CENTER_HEART_BEAT, req, opt)
    }

    pub fn heart_beat(&self, req: &super::center::HeartBeatReq) -> ::grpcio::Result<super::center::HeartBeatAck> {
        self.heart_beat_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heart_beat_async_opt(&self, req: &super::center::HeartBeatReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::HeartBeatAck>> {
        self.client.unary_call_async(&METHOD_GATE2_CENTER_HEART_BEAT, req, opt)
    }

    pub fn heart_beat_async(&self, req: &super::center::HeartBeatReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::HeartBeatAck>> {
        self.heart_beat_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Gate2Center {
    fn gate2_center_message(&mut self, ctx: ::grpcio::RpcContext, req: super::center::Gate2CenterReq, sink: ::grpcio::UnarySink<super::center::Gate2CenterAck>);
    fn login(&mut self, ctx: ::grpcio::RpcContext, req: super::center::LoginReq, sink: ::grpcio::UnarySink<super::center::LoginAck>);
    fn logout(&mut self, ctx: ::grpcio::RpcContext, req: super::center::LoginAck, sink: ::grpcio::UnarySink<super::center::LogoutAck>);
    fn heart_beat(&mut self, ctx: ::grpcio::RpcContext, req: super::center::HeartBeatReq, sink: ::grpcio::UnarySink<super::center::HeartBeatAck>);
}

pub fn create_gate2_center<S: Gate2Center + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_CENTER_GATE2_CENTER_MESSAGE, move |ctx, req, resp| {
        instance.gate2_center_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_CENTER_LOGIN, move |ctx, req, resp| {
        instance.login(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_CENTER_LOGOUT, move |ctx, req, resp| {
        instance.logout(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATE2_CENTER_HEART_BEAT, move |ctx, req, resp| {
        instance.heart_beat(ctx, req, resp)
    });
    builder.build()
}

const METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE: ::grpcio::Method<super::center::Center2GateReq, super::center::Center2GateAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Center2Gate/Center2GateMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CENTER2_GATE_KICK_USERS: ::grpcio::Method<super::center::CenterKickUserReq, super::center::CenterKickUserAck> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/message.Center2Gate/KickUsers",
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

    pub fn center2_gate_message_opt(&self, req: &super::center::Center2GateReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::Center2GateAck> {
        self.client.unary_call(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, req, opt)
    }

    pub fn center2_gate_message(&self, req: &super::center::Center2GateReq) -> ::grpcio::Result<super::center::Center2GateAck> {
        self.center2_gate_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn center2_gate_message_async_opt(&self, req: &super::center::Center2GateReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Center2GateAck>> {
        self.client.unary_call_async(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, req, opt)
    }

    pub fn center2_gate_message_async(&self, req: &super::center::Center2GateReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::Center2GateAck>> {
        self.center2_gate_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kick_users_opt(&self, req: &super::center::CenterKickUserReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::center::CenterKickUserAck> {
        self.client.unary_call(&METHOD_CENTER2_GATE_KICK_USERS, req, opt)
    }

    pub fn kick_users(&self, req: &super::center::CenterKickUserReq) -> ::grpcio::Result<super::center::CenterKickUserAck> {
        self.kick_users_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kick_users_async_opt(&self, req: &super::center::CenterKickUserReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::CenterKickUserAck>> {
        self.client.unary_call_async(&METHOD_CENTER2_GATE_KICK_USERS, req, opt)
    }

    pub fn kick_users_async(&self, req: &super::center::CenterKickUserReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::center::CenterKickUserAck>> {
        self.kick_users_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Center2Gate {
    fn center2_gate_message(&mut self, ctx: ::grpcio::RpcContext, req: super::center::Center2GateReq, sink: ::grpcio::UnarySink<super::center::Center2GateAck>);
    fn kick_users(&mut self, ctx: ::grpcio::RpcContext, req: super::center::CenterKickUserReq, sink: ::grpcio::UnarySink<super::center::CenterKickUserAck>);
}

pub fn create_center2_gate<S: Center2Gate + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CENTER2_GATE_CENTER2_GATE_MESSAGE, move |ctx, req, resp| {
        instance.center2_gate_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CENTER2_GATE_KICK_USERS, move |ctx, req, resp| {
        instance.kick_users(ctx, req, resp)
    });
    builder.build()
}
