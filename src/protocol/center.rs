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
//! Generated file from `center.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Gate2CenterRequest {
    // message fields
    pub user_id: i64,
    pub message: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gate2CenterRequest {
    fn default() -> &'a Gate2CenterRequest {
        <Gate2CenterRequest as ::protobuf::Message>::default_instance()
    }
}

impl Gate2CenterRequest {
    pub fn new() -> Gate2CenterRequest {
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

impl ::protobuf::Message for Gate2CenterRequest {
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

    fn new() -> Gate2CenterRequest {
        Gate2CenterRequest::new()
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
                    |m: &Gate2CenterRequest| { &m.user_id },
                    |m: &mut Gate2CenterRequest| { &mut m.user_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message",
                    |m: &Gate2CenterRequest| { &m.message },
                    |m: &mut Gate2CenterRequest| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gate2CenterRequest>(
                    "Gate2CenterRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Gate2CenterRequest {
        static mut instance: ::protobuf::lazy::Lazy<Gate2CenterRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gate2CenterRequest,
        };
        unsafe {
            instance.get(Gate2CenterRequest::new)
        }
    }
}

impl ::protobuf::Clear for Gate2CenterRequest {
    fn clear(&mut self) {
        self.user_id = 0;
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gate2CenterRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gate2CenterRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Gate2CenterResponse {
    // message fields
    pub can: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gate2CenterResponse {
    fn default() -> &'a Gate2CenterResponse {
        <Gate2CenterResponse as ::protobuf::Message>::default_instance()
    }
}

impl Gate2CenterResponse {
    pub fn new() -> Gate2CenterResponse {
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

impl ::protobuf::Message for Gate2CenterResponse {
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

    fn new() -> Gate2CenterResponse {
        Gate2CenterResponse::new()
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
                    |m: &Gate2CenterResponse| { &m.can },
                    |m: &mut Gate2CenterResponse| { &mut m.can },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gate2CenterResponse>(
                    "Gate2CenterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Gate2CenterResponse {
        static mut instance: ::protobuf::lazy::Lazy<Gate2CenterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gate2CenterResponse,
        };
        unsafe {
            instance.get(Gate2CenterResponse::new)
        }
    }
}

impl ::protobuf::Clear for Gate2CenterResponse {
    fn clear(&mut self) {
        self.can = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gate2CenterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gate2CenterResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Center2GateRequest {
    // message fields
    pub user_id: ::std::vec::Vec<i64>,
    pub message: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Center2GateRequest {
    fn default() -> &'a Center2GateRequest {
        <Center2GateRequest as ::protobuf::Message>::default_instance()
    }
}

impl Center2GateRequest {
    pub fn new() -> Center2GateRequest {
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

impl ::protobuf::Message for Center2GateRequest {
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

    fn new() -> Center2GateRequest {
        Center2GateRequest::new()
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
                    |m: &Center2GateRequest| { &m.user_id },
                    |m: &mut Center2GateRequest| { &mut m.user_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message",
                    |m: &Center2GateRequest| { &m.message },
                    |m: &mut Center2GateRequest| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Center2GateRequest>(
                    "Center2GateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Center2GateRequest {
        static mut instance: ::protobuf::lazy::Lazy<Center2GateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Center2GateRequest,
        };
        unsafe {
            instance.get(Center2GateRequest::new)
        }
    }
}

impl ::protobuf::Clear for Center2GateRequest {
    fn clear(&mut self) {
        self.user_id.clear();
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Center2GateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Center2GateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Center2GateResponse {
    // message fields
    pub user_id: ::std::vec::Vec<i64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Center2GateResponse {
    fn default() -> &'a Center2GateResponse {
        <Center2GateResponse as ::protobuf::Message>::default_instance()
    }
}

impl Center2GateResponse {
    pub fn new() -> Center2GateResponse {
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

impl ::protobuf::Message for Center2GateResponse {
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

    fn new() -> Center2GateResponse {
        Center2GateResponse::new()
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
                    |m: &Center2GateResponse| { &m.user_id },
                    |m: &mut Center2GateResponse| { &mut m.user_id },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Center2GateResponse>(
                    "Center2GateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Center2GateResponse {
        static mut instance: ::protobuf::lazy::Lazy<Center2GateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Center2GateResponse,
        };
        unsafe {
            instance.get(Center2GateResponse::new)
        }
    }
}

impl ::protobuf::Clear for Center2GateResponse {
    fn clear(&mut self) {
        self.user_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Center2GateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Center2GateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccenter.proto\x12\x07message\"G\n\x12Gate2CenterRequest\x12\x17\n\
    \x07user_id\x18\x01\x20\x01(\x03R\x06userId\x12\x18\n\x07message\x18\x02\
    \x20\x01(\x0cR\x07message\"'\n\x13Gate2CenterResponse\x12\x10\n\x03can\
    \x18\x01\x20\x01(\x08R\x03can\"G\n\x12Center2GateRequest\x12\x17\n\x07us\
    er_id\x18\x01\x20\x03(\x03R\x06userId\x12\x18\n\x07message\x18\x02\x20\
    \x01(\x0cR\x07message\".\n\x13Center2GateResponse\x12\x17\n\x07user_id\
    \x18\x01\x20\x03(\x03R\x06userId2^\n\x0bGate2Center\x12O\n\x12Gate2Cente\
    rMessage\x12\x1b.message.Gate2CenterRequest\x1a\x1c.message.Gate2CenterR\
    esponse2^\n\x0bCenter2Gate\x12O\n\x12Center2GateMessage\x12\x1b.message.\
    Center2GateRequest\x1a\x1c.message.Center2GateResponseJ\x8f\x06\n\x06\
    \x12\x04\0\0\x1d\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x02\x08\x0f\n\n\n\x02\x06\0\x12\x04\x04\0\x06\x03\n\n\n\x03\x06\0\
    \x01\x12\x03\x04\x08\x13\n\x0b\n\x04\x06\0\x02\0\x12\x03\x05\x04N\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03\x05\x08\x1a\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x05\x1c.\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x059L\n\n\n\x02\
    \x04\0\x12\x04\x08\x02\x0b\x03\n\n\n\x03\x04\0\x01\x12\x03\x08\n\x1c\n\
    \x17\n\x04\x04\0\x02\0\x12\x03\t\x08\x1a\"\n\xe7\x94\xa8\xe6\x88\xb7id\r\
    \n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x08\x08\x1e\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\t\x08\r\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x0e\x15\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x18\x19\n\x18\n\x04\x04\0\x02\x01\
    \x12\x03\n\x08\x1a\"\x0b\xe6\xb6\x88\xe6\x81\xaf\xe4\xbd\x93\r\n\n\r\n\
    \x05\x04\0\x02\x01\x04\x12\x04\n\x08\t\x1a\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\n\x08\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\x0e\x15\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\n\x18\x19\n\n\n\x02\x04\x01\x12\x04\r\x02\
    \x0f\x03\n\n\n\x03\x04\x01\x01\x12\x03\r\n\x1d\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x0e\x06\x13\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0e\x06\r\x1e\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0e\x06\n\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x0e\x0b\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0e\x11\
    \x12\n\n\n\x02\x06\x01\x12\x04\x12\0\x14\x01\n\n\n\x03\x06\x01\x01\x12\
    \x03\x12\x08\x13\n\x0b\n\x04\x06\x01\x02\0\x12\x03\x13\x04N\n\x0c\n\x05\
    \x06\x01\x02\0\x01\x12\x03\x13\x08\x1a\n\x0c\n\x05\x06\x01\x02\0\x02\x12\
    \x03\x13\x1c.\n\x0c\n\x05\x06\x01\x02\0\x03\x12\x03\x139L\n\n\n\x02\x04\
    \x02\x12\x04\x16\0\x19\x01\n\n\n\x03\x04\x02\x01\x12\x03\x16\x08\x1a\n&\
    \n\x04\x04\x02\x02\0\x12\x03\x17\x04$\"\x19\xe9\x9c\x80\xe8\xa6\x81\xe8\
    \xbd\xac\xe5\x8f\x91\xe7\x9a\x84\xe7\x94\xa8\xe6\x88\xb7id\r\n\n\x0c\n\
    \x05\x04\x02\x02\0\x04\x12\x03\x17\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x05\
    \x12\x03\x17\x10\x15\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x17\x16\x1d\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x17\"#\n\x15\n\x04\x04\x02\x02\x01\
    \x12\x03\x18\x04$\"\x08\xe6\xb6\x88\xe6\x81\xaf\r\n\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04\x18\x04\x17$\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03\x18\x04\t\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x18\n\x11\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03\x18\"#\n\n\n\x02\x04\x03\x12\x04\x1b\0\
    \x1d\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1b\x08\x1b\n\x20\n\x04\x04\x03\
    \x02\0\x12\x03\x1c\x04\x1f\"\x13\xe6\x88\x90\xe5\x8a\x9f\xe7\x9a\x84\xe7\
    \x94\xa8\xe6\x88\xb7id\r\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x1c\x04\
    \x0c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1c\r\x12\n\x0c\n\x05\x04\x03\
    \x02\0\x01\x12\x03\x1c\x13\x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1c\
    \x1d\x1eb\x06proto3\
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