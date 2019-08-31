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
//! Generated file from `node.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct NodeRegisterRequest {
    // message fields
    pub name: ::std::string::String,
    pub ip: ::std::string::String,
    pub port: ::std::string::String,
    pub cluster: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NodeRegisterRequest {
    fn default() -> &'a NodeRegisterRequest {
        <NodeRegisterRequest as ::protobuf::Message>::default_instance()
    }
}

impl NodeRegisterRequest {
    pub fn new() -> NodeRegisterRequest {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string ip = 2;


    pub fn get_ip(&self) -> &str {
        &self.ip
    }
    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip, ::std::string::String::new())
    }

    // string port = 3;


    pub fn get_port(&self) -> &str {
        &self.port
    }
    pub fn clear_port(&mut self) {
        self.port.clear();
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: ::std::string::String) {
        self.port = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_port(&mut self) -> &mut ::std::string::String {
        &mut self.port
    }

    // Take field
    pub fn take_port(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.port, ::std::string::String::new())
    }

    // string cluster = 4;


    pub fn get_cluster(&self) -> &str {
        &self.cluster
    }
    pub fn clear_cluster(&mut self) {
        self.cluster.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: ::std::string::String) {
        self.cluster = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster(&mut self) -> &mut ::std::string::String {
        &mut self.cluster
    }

    // Take field
    pub fn take_cluster(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cluster, ::std::string::String::new())
    }
}

impl ::protobuf::Message for NodeRegisterRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.port)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cluster)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ip);
        }
        if !self.port.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.port);
        }
        if !self.cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.cluster);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.ip.is_empty() {
            os.write_string(2, &self.ip)?;
        }
        if !self.port.is_empty() {
            os.write_string(3, &self.port)?;
        }
        if !self.cluster.is_empty() {
            os.write_string(4, &self.cluster)?;
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

    fn new() -> NodeRegisterRequest {
        NodeRegisterRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &NodeRegisterRequest| { &m.name },
                    |m: &mut NodeRegisterRequest| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    |m: &NodeRegisterRequest| { &m.ip },
                    |m: &mut NodeRegisterRequest| { &mut m.ip },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "port",
                    |m: &NodeRegisterRequest| { &m.port },
                    |m: &mut NodeRegisterRequest| { &mut m.port },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster",
                    |m: &NodeRegisterRequest| { &m.cluster },
                    |m: &mut NodeRegisterRequest| { &mut m.cluster },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeRegisterRequest>(
                    "NodeRegisterRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static NodeRegisterRequest {
        static mut instance: ::protobuf::lazy::Lazy<NodeRegisterRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeRegisterRequest,
        };
        unsafe {
            instance.get(NodeRegisterRequest::new)
        }
    }
}

impl ::protobuf::Clear for NodeRegisterRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.ip.clear();
        self.port.clear();
        self.cluster.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeRegisterRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeRegisterRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeRegisterResponse {
    // message fields
    pub err: ::std::string::String,
    pub node: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NodeRegisterResponse {
    fn default() -> &'a NodeRegisterResponse {
        <NodeRegisterResponse as ::protobuf::Message>::default_instance()
    }
}

impl NodeRegisterResponse {
    pub fn new() -> NodeRegisterResponse {
        ::std::default::Default::default()
    }

    // string err = 1;


    pub fn get_err(&self) -> &str {
        &self.err
    }
    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err(&mut self) -> &mut ::std::string::String {
        &mut self.err
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.err, ::std::string::String::new())
    }

    // string node = 2;


    pub fn get_node(&self) -> &str {
        &self.node
    }
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::std::string::String) {
        self.node = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut ::std::string::String {
        &mut self.node
    }

    // Take field
    pub fn take_node(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node, ::std::string::String::new())
    }
}

impl ::protobuf::Message for NodeRegisterResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.err)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node)?;
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
        if !self.err.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.err);
        }
        if !self.node.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.node);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.err.is_empty() {
            os.write_string(1, &self.err)?;
        }
        if !self.node.is_empty() {
            os.write_string(2, &self.node)?;
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

    fn new() -> NodeRegisterResponse {
        NodeRegisterResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "err",
                    |m: &NodeRegisterResponse| { &m.err },
                    |m: &mut NodeRegisterResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node",
                    |m: &NodeRegisterResponse| { &m.node },
                    |m: &mut NodeRegisterResponse| { &mut m.node },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeRegisterResponse>(
                    "NodeRegisterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static NodeRegisterResponse {
        static mut instance: ::protobuf::lazy::Lazy<NodeRegisterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeRegisterResponse,
        };
        unsafe {
            instance.get(NodeRegisterResponse::new)
        }
    }
}

impl ::protobuf::Clear for NodeRegisterResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.node.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeRegisterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeRegisterResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartRequest {
    // message fields
    pub time: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HeartRequest {
    fn default() -> &'a HeartRequest {
        <HeartRequest as ::protobuf::Message>::default_instance()
    }
}

impl HeartRequest {
    pub fn new() -> HeartRequest {
        ::std::default::Default::default()
    }

    // string time = 1;


    pub fn get_time(&self) -> &str {
        &self.time
    }
    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: ::std::string::String) {
        self.time = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut ::std::string::String {
        &mut self.time
    }

    // Take field
    pub fn take_time(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.time, ::std::string::String::new())
    }
}

impl ::protobuf::Message for HeartRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.time)?;
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
        if !self.time.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.time.is_empty() {
            os.write_string(1, &self.time)?;
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

    fn new() -> HeartRequest {
        HeartRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "time",
                    |m: &HeartRequest| { &m.time },
                    |m: &mut HeartRequest| { &mut m.time },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeartRequest>(
                    "HeartRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HeartRequest {
        static mut instance: ::protobuf::lazy::Lazy<HeartRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartRequest,
        };
        unsafe {
            instance.get(HeartRequest::new)
        }
    }
}

impl ::protobuf::Clear for HeartRequest {
    fn clear(&mut self) {
        self.time.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeartRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartResponse {
    // message fields
    pub err: ::std::string::String,
    pub time: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HeartResponse {
    fn default() -> &'a HeartResponse {
        <HeartResponse as ::protobuf::Message>::default_instance()
    }
}

impl HeartResponse {
    pub fn new() -> HeartResponse {
        ::std::default::Default::default()
    }

    // string err = 1;


    pub fn get_err(&self) -> &str {
        &self.err
    }
    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err(&mut self) -> &mut ::std::string::String {
        &mut self.err
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.err, ::std::string::String::new())
    }

    // string time = 2;


    pub fn get_time(&self) -> &str {
        &self.time
    }
    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: ::std::string::String) {
        self.time = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut ::std::string::String {
        &mut self.time
    }

    // Take field
    pub fn take_time(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.time, ::std::string::String::new())
    }
}

impl ::protobuf::Message for HeartResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.err)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.time)?;
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
        if !self.err.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.err);
        }
        if !self.time.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.err.is_empty() {
            os.write_string(1, &self.err)?;
        }
        if !self.time.is_empty() {
            os.write_string(2, &self.time)?;
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

    fn new() -> HeartResponse {
        HeartResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "err",
                    |m: &HeartResponse| { &m.err },
                    |m: &mut HeartResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "time",
                    |m: &HeartResponse| { &m.time },
                    |m: &mut HeartResponse| { &mut m.time },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeartResponse>(
                    "HeartResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HeartResponse {
        static mut instance: ::protobuf::lazy::Lazy<HeartResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartResponse,
        };
        unsafe {
            instance.get(HeartResponse::new)
        }
    }
}

impl ::protobuf::Clear for HeartResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.time.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeartResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nnode.proto\"g\n\x13NodeRegisterRequest\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x12\x0e\n\x02ip\x18\x02\x20\x01(\tR\x02ip\x12\x12\n\x04\
    port\x18\x03\x20\x01(\tR\x04port\x12\x18\n\x07cluster\x18\x04\x20\x01(\t\
    R\x07cluster\"<\n\x14NodeRegisterResponse\x12\x10\n\x03err\x18\x01\x20\
    \x01(\tR\x03err\x12\x12\n\x04node\x18\x02\x20\x01(\tR\x04node\"\"\n\x0cH\
    eartRequest\x12\x12\n\x04time\x18\x01\x20\x01(\tR\x04time\"5\n\rHeartRes\
    ponse\x12\x10\n\x03err\x18\x01\x20\x01(\tR\x03err\x12\x12\n\x04time\x18\
    \x02\x20\x01(\tR\x04time2g\n\x04Node\x127\n\x08Register\x12\x14.NodeRegi\
    sterRequest\x1a\x15.NodeRegisterResponse\x12&\n\x05Heart\x12\r.HeartRequ\
    est\x1a\x0e.HeartResponseb\x06proto3\
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
