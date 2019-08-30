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
//! Generated file from `service_discovery.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct DiscoveryRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DiscoveryRequest {
    fn default() -> &'a DiscoveryRequest {
        <DiscoveryRequest as ::protobuf::Message>::default_instance()
    }
}

impl DiscoveryRequest {
    pub fn new() -> DiscoveryRequest {
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
}

impl ::protobuf::Message for DiscoveryRequest {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

    fn new() -> DiscoveryRequest {
        DiscoveryRequest::new()
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
                    |m: &DiscoveryRequest| { &m.name },
                    |m: &mut DiscoveryRequest| { &mut m.name },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryRequest>(
                    "DiscoveryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DiscoveryRequest {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryRequest,
        };
        unsafe {
            instance.get(DiscoveryRequest::new)
        }
    }
}

impl ::protobuf::Clear for DiscoveryRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscoveryResponse {
    // message fields
    pub err: ::std::string::String,
    pub node: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DiscoveryResponse {
    fn default() -> &'a DiscoveryResponse {
        <DiscoveryResponse as ::protobuf::Message>::default_instance()
    }
}

impl DiscoveryResponse {
    pub fn new() -> DiscoveryResponse {
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

impl ::protobuf::Message for DiscoveryResponse {
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

    fn new() -> DiscoveryResponse {
        DiscoveryResponse::new()
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
                    |m: &DiscoveryResponse| { &m.err },
                    |m: &mut DiscoveryResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node",
                    |m: &DiscoveryResponse| { &m.node },
                    |m: &mut DiscoveryResponse| { &mut m.node },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryResponse>(
                    "DiscoveryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DiscoveryResponse {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryResponse,
        };
        unsafe {
            instance.get(DiscoveryResponse::new)
        }
    }
}

impl ::protobuf::Clear for DiscoveryResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.node.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscoveryNodeRequest {
    // message fields
    pub node: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DiscoveryNodeRequest {
    fn default() -> &'a DiscoveryNodeRequest {
        <DiscoveryNodeRequest as ::protobuf::Message>::default_instance()
    }
}

impl DiscoveryNodeRequest {
    pub fn new() -> DiscoveryNodeRequest {
        ::std::default::Default::default()
    }

    // string node = 1;


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

impl ::protobuf::Message for DiscoveryNodeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if !self.node.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.node);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.node.is_empty() {
            os.write_string(1, &self.node)?;
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

    fn new() -> DiscoveryNodeRequest {
        DiscoveryNodeRequest::new()
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
                    "node",
                    |m: &DiscoveryNodeRequest| { &m.node },
                    |m: &mut DiscoveryNodeRequest| { &mut m.node },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryNodeRequest>(
                    "DiscoveryNodeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DiscoveryNodeRequest {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryNodeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryNodeRequest,
        };
        unsafe {
            instance.get(DiscoveryNodeRequest::new)
        }
    }
}

impl ::protobuf::Clear for DiscoveryNodeRequest {
    fn clear(&mut self) {
        self.node.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryNodeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryNodeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscoveryNodeResponse {
    // message fields
    pub err: ::std::string::String,
    pub ip: ::std::string::String,
    pub addr: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DiscoveryNodeResponse {
    fn default() -> &'a DiscoveryNodeResponse {
        <DiscoveryNodeResponse as ::protobuf::Message>::default_instance()
    }
}

impl DiscoveryNodeResponse {
    pub fn new() -> DiscoveryNodeResponse {
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

    // string addr = 3;


    pub fn get_addr(&self) -> &str {
        &self.addr
    }
    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: ::std::string::String) {
        self.addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.addr, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DiscoveryNodeResponse {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.addr)?;
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
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ip);
        }
        if !self.addr.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.err.is_empty() {
            os.write_string(1, &self.err)?;
        }
        if !self.ip.is_empty() {
            os.write_string(2, &self.ip)?;
        }
        if !self.addr.is_empty() {
            os.write_string(3, &self.addr)?;
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

    fn new() -> DiscoveryNodeResponse {
        DiscoveryNodeResponse::new()
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
                    |m: &DiscoveryNodeResponse| { &m.err },
                    |m: &mut DiscoveryNodeResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    |m: &DiscoveryNodeResponse| { &m.ip },
                    |m: &mut DiscoveryNodeResponse| { &mut m.ip },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addr",
                    |m: &DiscoveryNodeResponse| { &m.addr },
                    |m: &mut DiscoveryNodeResponse| { &mut m.addr },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryNodeResponse>(
                    "DiscoveryNodeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DiscoveryNodeResponse {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryNodeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryNodeResponse,
        };
        unsafe {
            instance.get(DiscoveryNodeResponse::new)
        }
    }
}

impl ::protobuf::Clear for DiscoveryNodeResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.ip.clear();
        self.addr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryNodeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryNodeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17service_discovery.proto\"&\n\x10DiscoveryRequest\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\"9\n\x11DiscoveryResponse\x12\x10\n\x03err\
    \x18\x01\x20\x01(\tR\x03err\x12\x12\n\x04node\x18\x02\x20\x01(\tR\x04nod\
    e\"*\n\x14DiscoveryNodeRequest\x12\x12\n\x04node\x18\x01\x20\x01(\tR\x04\
    node\"M\n\x15DiscoveryNodeResponse\x12\x10\n\x03err\x18\x01\x20\x01(\tR\
    \x03err\x12\x0e\n\x02ip\x18\x02\x20\x01(\tR\x02ip\x12\x12\n\x04addr\x18\
    \x03\x20\x01(\tR\x04addr2\x86\x01\n\x10ServiceDiscovery\x122\n\tDiscover\
    y\x12\x11.DiscoveryRequest\x1a\x12.DiscoveryResponse\x12>\n\rDiscoveryNo\
    de\x12\x15.DiscoveryNodeRequest\x1a\x16.DiscoveryNodeResponseb\x06proto3\
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
