// This file is generated by rust-protobuf 2.24.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `steammessages_gamenetworking.steamclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct CGameNetworking_AllocateFakeIP_Request {
    // message fields
    app_id: ::std::option::Option<u32>,
    num_fake_ports: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_AllocateFakeIP_Request {
    fn default() -> &'a CGameNetworking_AllocateFakeIP_Request {
        <CGameNetworking_AllocateFakeIP_Request as ::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_AllocateFakeIP_Request {
    pub fn new() -> CGameNetworking_AllocateFakeIP_Request {
        ::std::default::Default::default()
    }

    // optional uint32 app_id = 1;


    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }
    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    // optional uint32 num_fake_ports = 2;


    pub fn get_num_fake_ports(&self) -> u32 {
        self.num_fake_ports.unwrap_or(0)
    }
    pub fn clear_num_fake_ports(&mut self) {
        self.num_fake_ports = ::std::option::Option::None;
    }

    pub fn has_num_fake_ports(&self) -> bool {
        self.num_fake_ports.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_fake_ports(&mut self, v: u32) {
        self.num_fake_ports = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for CGameNetworking_AllocateFakeIP_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_fake_ports = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_fake_ports {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.num_fake_ports {
            os.write_uint32(2, v)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CGameNetworking_AllocateFakeIP_Request {
        CGameNetworking_AllocateFakeIP_Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "app_id",
                |m: &CGameNetworking_AllocateFakeIP_Request| { &m.app_id },
                |m: &mut CGameNetworking_AllocateFakeIP_Request| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "num_fake_ports",
                |m: &CGameNetworking_AllocateFakeIP_Request| { &m.num_fake_ports },
                |m: &mut CGameNetworking_AllocateFakeIP_Request| { &mut m.num_fake_ports },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CGameNetworking_AllocateFakeIP_Request>(
                "CGameNetworking_AllocateFakeIP_Request",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CGameNetworking_AllocateFakeIP_Request {
        static instance: ::protobuf::rt::LazyV2<CGameNetworking_AllocateFakeIP_Request> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CGameNetworking_AllocateFakeIP_Request::new)
    }
}

impl ::protobuf::Clear for CGameNetworking_AllocateFakeIP_Request {
    fn clear(&mut self) {
        self.app_id = ::std::option::Option::None;
        self.num_fake_ports = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameNetworking_AllocateFakeIP_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameNetworking_AllocateFakeIP_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameNetworking_AllocateFakeIP_Response {
    // message fields
    fake_ip: ::std::option::Option<u32>,
    pub fake_ports: ::std::vec::Vec<u32>,
    renew_seconds: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_AllocateFakeIP_Response {
    fn default() -> &'a CGameNetworking_AllocateFakeIP_Response {
        <CGameNetworking_AllocateFakeIP_Response as ::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_AllocateFakeIP_Response {
    pub fn new() -> CGameNetworking_AllocateFakeIP_Response {
        ::std::default::Default::default()
    }

    // optional fixed32 fake_ip = 1;


    pub fn get_fake_ip(&self) -> u32 {
        self.fake_ip.unwrap_or(0)
    }
    pub fn clear_fake_ip(&mut self) {
        self.fake_ip = ::std::option::Option::None;
    }

    pub fn has_fake_ip(&self) -> bool {
        self.fake_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fake_ip(&mut self, v: u32) {
        self.fake_ip = ::std::option::Option::Some(v);
    }

    // repeated uint32 fake_ports = 2;


    pub fn get_fake_ports(&self) -> &[u32] {
        &self.fake_ports
    }
    pub fn clear_fake_ports(&mut self) {
        self.fake_ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_fake_ports(&mut self, v: ::std::vec::Vec<u32>) {
        self.fake_ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fake_ports(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fake_ports
    }

    // Take field
    pub fn take_fake_ports(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.fake_ports, ::std::vec::Vec::new())
    }

    // optional uint32 renew_seconds = 3;


    pub fn get_renew_seconds(&self) -> u32 {
        self.renew_seconds.unwrap_or(0)
    }
    pub fn clear_renew_seconds(&mut self) {
        self.renew_seconds = ::std::option::Option::None;
    }

    pub fn has_renew_seconds(&self) -> bool {
        self.renew_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renew_seconds(&mut self, v: u32) {
        self.renew_seconds = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for CGameNetworking_AllocateFakeIP_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.fake_ip = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.fake_ports)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.renew_seconds = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.fake_ip {
            my_size += 5;
        }
        for value in &self.fake_ports {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.renew_seconds {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fake_ip {
            os.write_fixed32(1, v)?;
        }
        for v in &self.fake_ports {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.renew_seconds {
            os.write_uint32(3, v)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CGameNetworking_AllocateFakeIP_Response {
        CGameNetworking_AllocateFakeIP_Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                "fake_ip",
                |m: &CGameNetworking_AllocateFakeIP_Response| { &m.fake_ip },
                |m: &mut CGameNetworking_AllocateFakeIP_Response| { &mut m.fake_ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "fake_ports",
                |m: &CGameNetworking_AllocateFakeIP_Response| { &m.fake_ports },
                |m: &mut CGameNetworking_AllocateFakeIP_Response| { &mut m.fake_ports },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "renew_seconds",
                |m: &CGameNetworking_AllocateFakeIP_Response| { &m.renew_seconds },
                |m: &mut CGameNetworking_AllocateFakeIP_Response| { &mut m.renew_seconds },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CGameNetworking_AllocateFakeIP_Response>(
                "CGameNetworking_AllocateFakeIP_Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CGameNetworking_AllocateFakeIP_Response {
        static instance: ::protobuf::rt::LazyV2<CGameNetworking_AllocateFakeIP_Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CGameNetworking_AllocateFakeIP_Response::new)
    }
}

impl ::protobuf::Clear for CGameNetworking_AllocateFakeIP_Response {
    fn clear(&mut self) {
        self.fake_ip = ::std::option::Option::None;
        self.fake_ports.clear();
        self.renew_seconds = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameNetworking_AllocateFakeIP_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameNetworking_AllocateFakeIP_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameNetworking_RenewFakeIP_Request {
    // message fields
    app_id: ::std::option::Option<u32>,
    fake_ip: ::std::option::Option<u32>,
    pub fake_ports: ::std::vec::Vec<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_RenewFakeIP_Request {
    fn default() -> &'a CGameNetworking_RenewFakeIP_Request {
        <CGameNetworking_RenewFakeIP_Request as ::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_RenewFakeIP_Request {
    pub fn new() -> CGameNetworking_RenewFakeIP_Request {
        ::std::default::Default::default()
    }

    // optional uint32 app_id = 1;


    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }
    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    // optional fixed32 fake_ip = 2;


    pub fn get_fake_ip(&self) -> u32 {
        self.fake_ip.unwrap_or(0)
    }
    pub fn clear_fake_ip(&mut self) {
        self.fake_ip = ::std::option::Option::None;
    }

    pub fn has_fake_ip(&self) -> bool {
        self.fake_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fake_ip(&mut self, v: u32) {
        self.fake_ip = ::std::option::Option::Some(v);
    }

    // repeated uint32 fake_ports = 3;


    pub fn get_fake_ports(&self) -> &[u32] {
        &self.fake_ports
    }
    pub fn clear_fake_ports(&mut self) {
        self.fake_ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_fake_ports(&mut self, v: ::std::vec::Vec<u32>) {
        self.fake_ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fake_ports(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fake_ports
    }

    // Take field
    pub fn take_fake_ports(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.fake_ports, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CGameNetworking_RenewFakeIP_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.fake_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.fake_ports)?;
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
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fake_ip {
            my_size += 5;
        }
        for value in &self.fake_ports {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.fake_ip {
            os.write_fixed32(2, v)?;
        }
        for v in &self.fake_ports {
            os.write_uint32(3, *v)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CGameNetworking_RenewFakeIP_Request {
        CGameNetworking_RenewFakeIP_Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "app_id",
                |m: &CGameNetworking_RenewFakeIP_Request| { &m.app_id },
                |m: &mut CGameNetworking_RenewFakeIP_Request| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                "fake_ip",
                |m: &CGameNetworking_RenewFakeIP_Request| { &m.fake_ip },
                |m: &mut CGameNetworking_RenewFakeIP_Request| { &mut m.fake_ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "fake_ports",
                |m: &CGameNetworking_RenewFakeIP_Request| { &m.fake_ports },
                |m: &mut CGameNetworking_RenewFakeIP_Request| { &mut m.fake_ports },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CGameNetworking_RenewFakeIP_Request>(
                "CGameNetworking_RenewFakeIP_Request",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CGameNetworking_RenewFakeIP_Request {
        static instance: ::protobuf::rt::LazyV2<CGameNetworking_RenewFakeIP_Request> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CGameNetworking_RenewFakeIP_Request::new)
    }
}

impl ::protobuf::Clear for CGameNetworking_RenewFakeIP_Request {
    fn clear(&mut self) {
        self.app_id = ::std::option::Option::None;
        self.fake_ip = ::std::option::Option::None;
        self.fake_ports.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameNetworking_RenewFakeIP_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameNetworking_RenewFakeIP_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameNetworking_ReleaseFakeIP_Notification {
    // message fields
    app_id: ::std::option::Option<u32>,
    fake_ip: ::std::option::Option<u32>,
    pub fake_ports: ::std::vec::Vec<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_ReleaseFakeIP_Notification {
    fn default() -> &'a CGameNetworking_ReleaseFakeIP_Notification {
        <CGameNetworking_ReleaseFakeIP_Notification as ::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_ReleaseFakeIP_Notification {
    pub fn new() -> CGameNetworking_ReleaseFakeIP_Notification {
        ::std::default::Default::default()
    }

    // optional uint32 app_id = 1;


    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }
    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    // optional fixed32 fake_ip = 2;


    pub fn get_fake_ip(&self) -> u32 {
        self.fake_ip.unwrap_or(0)
    }
    pub fn clear_fake_ip(&mut self) {
        self.fake_ip = ::std::option::Option::None;
    }

    pub fn has_fake_ip(&self) -> bool {
        self.fake_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fake_ip(&mut self, v: u32) {
        self.fake_ip = ::std::option::Option::Some(v);
    }

    // repeated uint32 fake_ports = 3;


    pub fn get_fake_ports(&self) -> &[u32] {
        &self.fake_ports
    }
    pub fn clear_fake_ports(&mut self) {
        self.fake_ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_fake_ports(&mut self, v: ::std::vec::Vec<u32>) {
        self.fake_ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fake_ports(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fake_ports
    }

    // Take field
    pub fn take_fake_ports(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.fake_ports, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CGameNetworking_ReleaseFakeIP_Notification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.fake_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.fake_ports)?;
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
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fake_ip {
            my_size += 5;
        }
        for value in &self.fake_ports {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.fake_ip {
            os.write_fixed32(2, v)?;
        }
        for v in &self.fake_ports {
            os.write_uint32(3, *v)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CGameNetworking_ReleaseFakeIP_Notification {
        CGameNetworking_ReleaseFakeIP_Notification::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "app_id",
                |m: &CGameNetworking_ReleaseFakeIP_Notification| { &m.app_id },
                |m: &mut CGameNetworking_ReleaseFakeIP_Notification| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                "fake_ip",
                |m: &CGameNetworking_ReleaseFakeIP_Notification| { &m.fake_ip },
                |m: &mut CGameNetworking_ReleaseFakeIP_Notification| { &mut m.fake_ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "fake_ports",
                |m: &CGameNetworking_ReleaseFakeIP_Notification| { &m.fake_ports },
                |m: &mut CGameNetworking_ReleaseFakeIP_Notification| { &mut m.fake_ports },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CGameNetworking_ReleaseFakeIP_Notification>(
                "CGameNetworking_ReleaseFakeIP_Notification",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CGameNetworking_ReleaseFakeIP_Notification {
        static instance: ::protobuf::rt::LazyV2<CGameNetworking_ReleaseFakeIP_Notification> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CGameNetworking_ReleaseFakeIP_Notification::new)
    }
}

impl ::protobuf::Clear for CGameNetworking_ReleaseFakeIP_Notification {
    fn clear(&mut self) {
        self.app_id = ::std::option::Option::None;
        self.fake_ip = ::std::option::Option::None;
        self.fake_ports.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameNetworking_ReleaseFakeIP_Notification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameNetworking_ReleaseFakeIP_Notification {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.steammessages_gamenetworking.steamclient.proto\x1a,steammessages_unif\
    ied_base.steamclient.proto\"e\n&CGameNetworking_AllocateFakeIP_Request\
    \x12\x15\n\x06app_id\x18\x01\x20\x01(\rR\x05appId\x12$\n\x0enum_fake_por\
    ts\x18\x02\x20\x01(\rR\x0cnumFakePorts\"\x86\x01\n'CGameNetworking_Alloc\
    ateFakeIP_Response\x12\x17\n\x07fake_ip\x18\x01\x20\x01(\x07R\x06fakeIp\
    \x12\x1d\n\nfake_ports\x18\x02\x20\x03(\rR\tfakePorts\x12#\n\rrenew_seco\
    nds\x18\x03\x20\x01(\rR\x0crenewSeconds\"t\n#CGameNetworking_RenewFakeIP\
    _Request\x12\x15\n\x06app_id\x18\x01\x20\x01(\rR\x05appId\x12\x17\n\x07f\
    ake_ip\x18\x02\x20\x01(\x07R\x06fakeIp\x12\x1d\n\nfake_ports\x18\x03\x20\
    \x03(\rR\tfakePorts\"{\n*CGameNetworking_ReleaseFakeIP_Notification\x12\
    \x15\n\x06app_id\x18\x01\x20\x01(\rR\x05appId\x12\x17\n\x07fake_ip\x18\
    \x02\x20\x01(\x07R\x06fakeIp\x12\x1d\n\nfake_ports\x18\x03\x20\x03(\rR\t\
    fakePorts2\xe0\x03\n\x0eGameNetworking\x12\x8c\x01\n\x0eAllocateFakeIP\
    \x12'.CGameNetworking_AllocateFakeIP_Request\x1a(.CGameNetworking_Alloca\
    teFakeIP_Response\"'\x82\xb5\x18#Client\x20is\x20asking\x20to\x20lease\
    \x20a\x20FakeIP.\x12\x8b\x01\n\x0bRenewFakeIP\x12$.CGameNetworking_Renew\
    FakeIP_Request\x1a(.CGameNetworking_AllocateFakeIP_Response\",\x82\xb5\
    \x18(Client\x20is\x20asking\x20to\x20reup\x20a\x20FakeIP\x20lease.\x12\
    \x86\x01\n\x13NotifyReleaseFakeIP\x12+.CGameNetworking_ReleaseFakeIP_Not\
    ification\x1a\x0b.NoResponse\"5\x82\xb5\x181Client\x20informs\x20server\
    \x20it\x20is\x20done\x20with\x20the\x20FakeIP.\x1a(\x82\xb5\x18$Services\
    \x20that\x20support\x20P2P\x20networkingB\x03\x80\x01\x01J\xb7\t\n\x06\
    \x12\x04\0\0)\x01\n\t\n\x02\x03\0\x12\x03\0\06\n\x08\n\x01\x08\x12\x03\
    \x02\0\"\n\t\n\x02\x08\x10\x12\x03\x02\0\"\n\n\n\x02\x04\0\x12\x04\x04\0\
    \x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08.\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x05\x08#\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x05\x08\x10\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\x05\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x05\x18\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05!\"\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x06\x08+\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x03\x06\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x11\x17\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03\x06\x18&\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x06)*\n\n\n\x02\x04\x01\x12\x04\t\0\r\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\t\x08/\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x08%\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03\n\x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\n\
    \x11\x18\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\x19\x20\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\n#$\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0b\x08'\
    \n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x0b\x08\x10\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x03\x0b\x11\x17\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\
    \x0b\x18\"\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0b%&\n\x0b\n\x04\x04\
    \x01\x02\x02\x12\x03\x0c\x08*\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\
    \x0c\x08\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x0c\x11\x17\n\x0c\n\
    \x05\x04\x01\x02\x02\x01\x12\x03\x0c\x18%\n\x0c\n\x05\x04\x01\x02\x02\
    \x03\x12\x03\x0c()\n\n\n\x02\x04\x02\x12\x04\x0f\0\x13\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x0f\x08+\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x10\x08#\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x10\x08\x10\n\x0c\n\x05\x04\x02\x02\
    \0\x05\x12\x03\x10\x11\x17\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x10\x18\
    \x1e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x10!\"\n\x0b\n\x04\x04\x02\
    \x02\x01\x12\x03\x11\x08%\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x11\
    \x08\x10\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x11\x11\x18\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03\x11\x19\x20\n\x0c\n\x05\x04\x02\x02\x01\x03\
    \x12\x03\x11#$\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x12\x08'\n\x0c\n\x05\
    \x04\x02\x02\x02\x04\x12\x03\x12\x08\x10\n\x0c\n\x05\x04\x02\x02\x02\x05\
    \x12\x03\x12\x11\x17\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x12\x18\"\n\
    \x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x12%&\n\n\n\x02\x04\x03\x12\x04\
    \x15\0\x19\x01\n\n\n\x03\x04\x03\x01\x12\x03\x15\x082\n\x0b\n\x04\x04\
    \x03\x02\0\x12\x03\x16\x08#\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x16\
    \x08\x10\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x16\x11\x17\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03\x16\x18\x1e\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x03\x16!\"\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x17\x08%\n\x0c\n\x05\x04\
    \x03\x02\x01\x04\x12\x03\x17\x08\x10\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\
    \x03\x17\x11\x18\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x17\x19\x20\n\
    \x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x17#$\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03\x18\x08'\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03\x18\x08\x10\n\
    \x0c\n\x05\x04\x03\x02\x02\x05\x12\x03\x18\x11\x17\n\x0c\n\x05\x04\x03\
    \x02\x02\x01\x12\x03\x18\x18\"\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\
    \x18%&\n\n\n\x02\x06\0\x12\x04\x1b\0)\x01\n\n\n\x03\x06\0\x01\x12\x03\
    \x1b\x08\x16\n\n\n\x03\x06\0\x03\x12\x03\x1c\x08N\n\r\n\x06\x06\0\x03\
    \xd0\x86\x03\x12\x03\x1c\x08N\n\x0c\n\x04\x06\0\x02\0\x12\x04\x1e\x08\
    \x20\t\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x1e\x0c\x1a\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03\x1e\x1cC\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x1eNv\n\
    \x0c\n\x05\x06\0\x02\0\x04\x12\x03\x1f\x10T\n\x0f\n\x08\x06\0\x02\0\x04\
    \xd0\x86\x03\x12\x03\x1f\x10T\n\x0c\n\x04\x06\0\x02\x01\x12\x04\"\x08$\t\
    \n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\"\x0c\x17\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\"\x19=\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\"Hp\n\x0c\
    \n\x05\x06\0\x02\x01\x04\x12\x03#\x10Y\n\x0f\n\x08\x06\0\x02\x01\x04\xd0\
    \x86\x03\x12\x03#\x10Y\n\x0c\n\x04\x06\0\x02\x02\x12\x04&\x08(\t\n\x0c\n\
    \x05\x06\0\x02\x02\x01\x12\x03&\x0c\x1f\n\x0c\n\x05\x06\0\x02\x02\x02\
    \x12\x03&!L\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03&Wb\n\x0c\n\x05\x06\0\
    \x02\x02\x04\x12\x03'\x10b\n\x0f\n\x08\x06\0\x02\x02\x04\xd0\x86\x03\x12\
    \x03'\x10b\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
