// This file is generated by rust-protobuf 2.8.0. Do not edit
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
//! Generated file from `room.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Gate2RoomReq {
    // message fields
    pub user_id: i64,
    pub message: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gate2RoomReq {
    fn default() -> &'a Gate2RoomReq {
        <Gate2RoomReq as ::protobuf::Message>::default_instance()
    }
}

impl Gate2RoomReq {
    pub fn new() -> Gate2RoomReq {
        ::std::default::Default::default()
    }

    // int64 user_id = 1;


    pub fn get_user_id(&self) -> i64 {
        self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: i64) {
        self.user_id = v;
    }

    // bytes message = 2;


    pub fn get_message(&self) -> &[u8] {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.message, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Gate2RoomReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.user_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.user_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.user_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.user_id != 0 {
            os.write_int64(1, self.user_id)?;
        }
        if !self.message.is_empty() {
            os.write_bytes(2, &self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Gate2RoomReq {
        Gate2RoomReq::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "user_id",
                    |m: &Gate2RoomReq| { &m.user_id },
                    |m: &mut Gate2RoomReq| { &mut m.user_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message",
                    |m: &Gate2RoomReq| { &m.message },
                    |m: &mut Gate2RoomReq| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gate2RoomReq>(
                    "Gate2RoomReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Gate2RoomReq {
        static mut instance: ::protobuf::lazy::Lazy<Gate2RoomReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gate2RoomReq,
        };
        unsafe {
            instance.get(Gate2RoomReq::new)
        }
    }
}

impl ::protobuf::Clear for Gate2RoomReq {
    fn clear(&mut self) {
        self.user_id = 0;
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gate2RoomReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gate2RoomReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Gate2RoomAck {
    // message fields
    pub can: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gate2RoomAck {
    fn default() -> &'a Gate2RoomAck {
        <Gate2RoomAck as ::protobuf::Message>::default_instance()
    }
}

impl Gate2RoomAck {
    pub fn new() -> Gate2RoomAck {
        ::std::default::Default::default()
    }

    // bool can = 1;


    pub fn get_can(&self) -> bool {
        self.can
    }
    pub fn clear_can(&mut self) {
        self.can = false;
    }

    // Param is passed by value, moved
    pub fn set_can(&mut self, v: bool) {
        self.can = v;
    }
}

impl ::protobuf::Message for Gate2RoomAck {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.can = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.can != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.can != false {
            os.write_bool(1, self.can)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Gate2RoomAck {
        Gate2RoomAck::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can",
                    |m: &Gate2RoomAck| { &m.can },
                    |m: &mut Gate2RoomAck| { &mut m.can },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gate2RoomAck>(
                    "Gate2RoomAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Gate2RoomAck {
        static mut instance: ::protobuf::lazy::Lazy<Gate2RoomAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gate2RoomAck,
        };
        unsafe {
            instance.get(Gate2RoomAck::new)
        }
    }
}

impl ::protobuf::Clear for Gate2RoomAck {
    fn clear(&mut self) {
        self.can = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gate2RoomAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gate2RoomAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Room2GateReq {
    // message fields
    pub user_id: ::std::vec::Vec<i64>,
    pub message: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Room2GateReq {
    fn default() -> &'a Room2GateReq {
        <Room2GateReq as ::protobuf::Message>::default_instance()
    }
}

impl Room2GateReq {
    pub fn new() -> Room2GateReq {
        ::std::default::Default::default()
    }

    // repeated int64 user_id = 1;


    pub fn get_user_id(&self) -> &[i64] {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::vec::Vec<i64>) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_id(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.user_id, ::std::vec::Vec::new())
    }

    // bytes message = 2;


    pub fn get_message(&self) -> &[u8] {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.message, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Room2GateReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.user_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.user_id {
            os.write_int64(1, *v)?;
        };
        if !self.message.is_empty() {
            os.write_bytes(2, &self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Room2GateReq {
        Room2GateReq::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "user_id",
                    |m: &Room2GateReq| { &m.user_id },
                    |m: &mut Room2GateReq| { &mut m.user_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message",
                    |m: &Room2GateReq| { &m.message },
                    |m: &mut Room2GateReq| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Room2GateReq>(
                    "Room2GateReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Room2GateReq {
        static mut instance: ::protobuf::lazy::Lazy<Room2GateReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Room2GateReq,
        };
        unsafe {
            instance.get(Room2GateReq::new)
        }
    }
}

impl ::protobuf::Clear for Room2GateReq {
    fn clear(&mut self) {
        self.user_id.clear();
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Room2GateReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Room2GateReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Room2GateAck {
    // message fields
    pub user_id: ::std::vec::Vec<i64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Room2GateAck {
    fn default() -> &'a Room2GateAck {
        <Room2GateAck as ::protobuf::Message>::default_instance()
    }
}

impl Room2GateAck {
    pub fn new() -> Room2GateAck {
        ::std::default::Default::default()
    }

    // repeated int64 user_id = 1;


    pub fn get_user_id(&self) -> &[i64] {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::vec::Vec<i64>) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_id(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.user_id, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Room2GateAck {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.user_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.user_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.user_id {
            os.write_int64(1, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Room2GateAck {
        Room2GateAck::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "user_id",
                    |m: &Room2GateAck| { &m.user_id },
                    |m: &mut Room2GateAck| { &mut m.user_id },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Room2GateAck>(
                    "Room2GateAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Room2GateAck {
        static mut instance: ::protobuf::lazy::Lazy<Room2GateAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Room2GateAck,
        };
        unsafe {
            instance.get(Room2GateAck::new)
        }
    }
}

impl ::protobuf::Clear for Room2GateAck {
    fn clear(&mut self) {
        self.user_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Room2GateAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Room2GateAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nroom.proto\x12\x08protocol\"A\n\x0cGate2RoomReq\x12\x17\n\x07user_id\
    \x18\x01\x20\x01(\x03R\x06userId\x12\x18\n\x07message\x18\x02\x20\x01(\
    \x0cR\x07message\"\x20\n\x0cGate2RoomAck\x12\x10\n\x03can\x18\x01\x20\
    \x01(\x08R\x03can\"A\n\x0cRoom2GateReq\x12\x17\n\x07user_id\x18\x01\x20\
    \x03(\x03R\x06userId\x12\x18\n\x07message\x18\x02\x20\x01(\x0cR\x07messa\
    ge\"'\n\x0cRoom2GateAck\x12\x17\n\x07user_id\x18\x01\x20\x03(\x03R\x06us\
    erId2O\n\tGate2Room\x12B\n\x10Gate2RoomMessage\x12\x16.protocol.Gate2Roo\
    mReq\x1a\x16.protocol.Gate2RoomAck2O\n\tRoom2Gate\x12B\n\x10Room2GateMes\
    sage\x12\x16.protocol.Room2GateReq\x1a\x16.protocol.Room2GateAckJ\xc1\
    \x06\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x02\x08\x10\n\n\n\x02\x06\0\x12\x04\x04\0\x06\x01\n\n\n\
    \x03\x06\0\x01\x12\x03\x04\x08\x11\n&\n\x04\x06\0\x02\0\x12\x03\x05\x02<\
    \"\x19\x20\xe5\xae\xa2\xe6\x88\xb7\xe7\xab\xaf-\xe6\x88\xbf\xe9\x97\xb4\
    \xe6\xb6\x88\xe6\x81\xaf\r\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x05\x06\
    \x16\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x05\x17#\n\x0c\n\x05\x06\0\x02\
    \0\x03\x12\x03\x05.:\n\n\n\x02\x04\0\x12\x04\x08\0\x0b\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x08\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02\x14\n\r\
    \n\x05\x04\0\x02\0\x04\x12\x04\t\x02\x08\x16\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\t\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x08\x0f\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\t\x12\x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \n\x02\x14\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\n\x02\t\x14\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\n\x08\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\x12\x13\n\n\n\x02\
    \x04\x01\x12\x04\r\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\r\x08\x14\n\
    \x1b\n\x04\x04\x01\x02\0\x12\x03\x0e\x02\x0f\"\x0e\xe6\x98\xaf\xe5\x90\
    \xa6\xe6\x88\x90\xe5\x8a\x9f\r\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0e\
    \x02\r\x16\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0e\x02\x06\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x0e\x07\n\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x0e\r\x0e\n\n\n\x02\x06\x01\x12\x04\x11\0\x13\x01\n\n\n\x03\x06\x01\
    \x01\x12\x03\x11\x08\x11\n%\n\x04\x06\x01\x02\0\x12\x03\x12\x02<\"\x18\
    \xe6\x88\xbf\xe9\x97\xb4-\xe5\xae\xa2\xe6\x88\xb7\xe7\xab\xaf\xe6\xb6\
    \x88\xe6\x81\xaf\r\n\n\x0c\n\x05\x06\x01\x02\0\x01\x12\x03\x12\x06\x16\n\
    \x0c\n\x05\x06\x01\x02\0\x02\x12\x03\x12\x17#\n\x0c\n\x05\x06\x01\x02\0\
    \x03\x12\x03\x12.:\n\n\n\x02\x04\x02\x12\x04\x15\0\x18\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x15\x08\x14\n&\n\x04\x04\x02\x02\0\x12\x03\x16\x02\x1d\
    \"\x19\xe9\x9c\x80\xe8\xa6\x81\xe8\xbd\xac\xe5\x8f\x91\xe7\x9a\x84\xe7\
    \x94\xa8\xe6\x88\xb7id\r\n\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x16\x02\
    \n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x16\x0b\x10\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\x16\x11\x18\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x16\
    \x1b\x1c\n\x15\n\x04\x04\x02\x02\x01\x12\x03\x17\x02\x14\"\x08\xe6\xb6\
    \x88\xe6\x81\xaf\r\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x17\x02\x16\
    \x1d\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x17\x02\x07\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x17\x08\x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x17\x12\x13\n\n\n\x02\x04\x03\x12\x04\x1a\0\x1c\x01\n\n\n\x03\x04\
    \x03\x01\x12\x03\x1a\x08\x14\n&\n\x04\x04\x03\x02\0\x12\x03\x1b\x02\x1d\
    \"\x19\xe8\xbd\xac\xe5\x8f\x91\xe6\x88\x90\xe5\x8a\x9f\xe7\x9a\x84\xe7\
    \x94\xa8\xe6\x88\xb7id\r\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x1b\x02\
    \n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1b\x0b\x10\n\x0c\n\x05\x04\x03\
    \x02\0\x01\x12\x03\x1b\x11\x18\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1b\
    \x1b\x1cb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
