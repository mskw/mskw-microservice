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
//! Generated file from `json_service.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct JsonServiceRegisterRequest {
    // message fields
    pub node: ::std::string::String,
    pub uri: ::std::string::String,
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a JsonServiceRegisterRequest {
    fn default() -> &'a JsonServiceRegisterRequest {
        <JsonServiceRegisterRequest as ::protobuf::Message>::default_instance()
    }
}

impl JsonServiceRegisterRequest {
    pub fn new() -> JsonServiceRegisterRequest {
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

    // string uri = 2;


    pub fn get_uri(&self) -> &str {
        &self.uri
    }
    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        &mut self.uri
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uri, ::std::string::String::new())
    }

    // string name = 3;


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

impl ::protobuf::Message for JsonServiceRegisterRequest {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.uri)?;
                },
                3 => {
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
        if !self.node.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.node);
        }
        if !self.uri.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.uri);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.node.is_empty() {
            os.write_string(1, &self.node)?;
        }
        if !self.uri.is_empty() {
            os.write_string(2, &self.uri)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
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

    fn new() -> JsonServiceRegisterRequest {
        JsonServiceRegisterRequest::new()
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
                    |m: &JsonServiceRegisterRequest| { &m.node },
                    |m: &mut JsonServiceRegisterRequest| { &mut m.node },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    |m: &JsonServiceRegisterRequest| { &m.uri },
                    |m: &mut JsonServiceRegisterRequest| { &mut m.uri },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &JsonServiceRegisterRequest| { &m.name },
                    |m: &mut JsonServiceRegisterRequest| { &mut m.name },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JsonServiceRegisterRequest>(
                    "JsonServiceRegisterRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static JsonServiceRegisterRequest {
        static mut instance: ::protobuf::lazy::Lazy<JsonServiceRegisterRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JsonServiceRegisterRequest,
        };
        unsafe {
            instance.get(JsonServiceRegisterRequest::new)
        }
    }
}

impl ::protobuf::Clear for JsonServiceRegisterRequest {
    fn clear(&mut self) {
        self.node.clear();
        self.uri.clear();
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JsonServiceRegisterRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JsonServiceRegisterRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JsonServiceRegisterResponse {
    // message fields
    pub err: ::std::string::String,
    pub uuid: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a JsonServiceRegisterResponse {
    fn default() -> &'a JsonServiceRegisterResponse {
        <JsonServiceRegisterResponse as ::protobuf::Message>::default_instance()
    }
}

impl JsonServiceRegisterResponse {
    pub fn new() -> JsonServiceRegisterResponse {
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

    // string uuid = 2;


    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::string::String) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::string::String {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uuid, ::std::string::String::new())
    }
}

impl ::protobuf::Message for JsonServiceRegisterResponse {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.err.is_empty() {
            os.write_string(1, &self.err)?;
        }
        if !self.uuid.is_empty() {
            os.write_string(2, &self.uuid)?;
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

    fn new() -> JsonServiceRegisterResponse {
        JsonServiceRegisterResponse::new()
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
                    |m: &JsonServiceRegisterResponse| { &m.err },
                    |m: &mut JsonServiceRegisterResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uuid",
                    |m: &JsonServiceRegisterResponse| { &m.uuid },
                    |m: &mut JsonServiceRegisterResponse| { &mut m.uuid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JsonServiceRegisterResponse>(
                    "JsonServiceRegisterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static JsonServiceRegisterResponse {
        static mut instance: ::protobuf::lazy::Lazy<JsonServiceRegisterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JsonServiceRegisterResponse,
        };
        unsafe {
            instance.get(JsonServiceRegisterResponse::new)
        }
    }
}

impl ::protobuf::Clear for JsonServiceRegisterResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.uuid.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JsonServiceRegisterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JsonServiceRegisterResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JsonRequest {
    // message fields
    pub name: ::std::string::String,
    pub json: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a JsonRequest {
    fn default() -> &'a JsonRequest {
        <JsonRequest as ::protobuf::Message>::default_instance()
    }
}

impl JsonRequest {
    pub fn new() -> JsonRequest {
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

    // string json = 2;


    pub fn get_json(&self) -> &str {
        &self.json
    }
    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::string::String) {
        self.json = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json(&mut self) -> &mut ::std::string::String {
        &mut self.json
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.json, ::std::string::String::new())
    }
}

impl ::protobuf::Message for JsonRequest {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.json)?;
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
        if !self.json.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.json);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.json.is_empty() {
            os.write_string(2, &self.json)?;
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

    fn new() -> JsonRequest {
        JsonRequest::new()
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
                    |m: &JsonRequest| { &m.name },
                    |m: &mut JsonRequest| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "json",
                    |m: &JsonRequest| { &m.json },
                    |m: &mut JsonRequest| { &mut m.json },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JsonRequest>(
                    "JsonRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static JsonRequest {
        static mut instance: ::protobuf::lazy::Lazy<JsonRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JsonRequest,
        };
        unsafe {
            instance.get(JsonRequest::new)
        }
    }
}

impl ::protobuf::Clear for JsonRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.json.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JsonRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JsonRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JsonResponse {
    // message fields
    pub err: ::std::string::String,
    pub json: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a JsonResponse {
    fn default() -> &'a JsonResponse {
        <JsonResponse as ::protobuf::Message>::default_instance()
    }
}

impl JsonResponse {
    pub fn new() -> JsonResponse {
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

    // string json = 2;


    pub fn get_json(&self) -> &str {
        &self.json
    }
    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::string::String) {
        self.json = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json(&mut self) -> &mut ::std::string::String {
        &mut self.json
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.json, ::std::string::String::new())
    }
}

impl ::protobuf::Message for JsonResponse {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.json)?;
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
        if !self.json.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.json);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.err.is_empty() {
            os.write_string(1, &self.err)?;
        }
        if !self.json.is_empty() {
            os.write_string(2, &self.json)?;
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

    fn new() -> JsonResponse {
        JsonResponse::new()
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
                    |m: &JsonResponse| { &m.err },
                    |m: &mut JsonResponse| { &mut m.err },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "json",
                    |m: &JsonResponse| { &m.json },
                    |m: &mut JsonResponse| { &mut m.json },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JsonResponse>(
                    "JsonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static JsonResponse {
        static mut instance: ::protobuf::lazy::Lazy<JsonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JsonResponse,
        };
        unsafe {
            instance.get(JsonResponse::new)
        }
    }
}

impl ::protobuf::Clear for JsonResponse {
    fn clear(&mut self) {
        self.err.clear();
        self.json.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JsonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JsonResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12json_service.proto\"V\n\x1aJsonServiceRegisterRequest\x12\x12\n\
    \x04node\x18\x01\x20\x01(\tR\x04node\x12\x10\n\x03uri\x18\x02\x20\x01(\t\
    R\x03uri\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\"C\n\x1bJsonServi\
    ceRegisterResponse\x12\x10\n\x03err\x18\x01\x20\x01(\tR\x03err\x12\x12\n\
    \x04uuid\x18\x02\x20\x01(\tR\x04uuid\"5\n\x0bJsonRequest\x12\x12\n\x04na\
    me\x18\x01\x20\x01(\tR\x04name\x12\x12\n\x04json\x18\x02\x20\x01(\tR\x04\
    json\"4\n\x0cJsonResponse\x12\x10\n\x03err\x18\x01\x20\x01(\tR\x03err\
    \x12\x12\n\x04json\x18\x02\x20\x01(\tR\x04json2\x80\x01\n\x0bJsonService\
    \x12L\n\x0fServiceRegister\x12\x1b.JsonServiceRegisterRequest\x1a\x1c.Js\
    onServiceRegisterResponse\x12#\n\x04Call\x12\x0c.JsonRequest\x1a\r.JsonR\
    esponseb\x06proto3\
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
