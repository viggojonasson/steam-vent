// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steammessages_unified_base.steamclient.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NoResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NoResponse {
    // special fields
    // @@protoc_insertion_point(special_field:NoResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NoResponse {
    fn default() -> &'a NoResponse {
        <NoResponse as ::protobuf::Message>::default_instance()
    }
}

impl NoResponse {
    pub fn new() -> NoResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for NoResponse {
    const NAME: &'static str = "NoResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NoResponse {
        NoResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NoResponse {
        static instance: NoResponse = NoResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EProtoExecutionSite)
pub enum EProtoExecutionSite {
    // @@protoc_insertion_point(enum_value:EProtoExecutionSite.k_EProtoExecutionSiteUnknown)
    k_EProtoExecutionSiteUnknown = 0,
    // @@protoc_insertion_point(enum_value:EProtoExecutionSite.k_EProtoExecutionSiteSteamClient)
    k_EProtoExecutionSiteSteamClient = 2,
}

impl ::protobuf::Enum for EProtoExecutionSite {
    const NAME: &'static str = "EProtoExecutionSite";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EProtoExecutionSite> {
        match value {
            0 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            2 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EProtoExecutionSite> {
        match str {
            "k_EProtoExecutionSiteUnknown" => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            "k_EProtoExecutionSiteSteamClient" => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EProtoExecutionSite] = &[
        EProtoExecutionSite::k_EProtoExecutionSiteUnknown,
        EProtoExecutionSite::k_EProtoExecutionSiteSteamClient,
    ];
}

impl ::std::default::Default for EProtoExecutionSite {
    fn default() -> Self {
        EProtoExecutionSite::k_EProtoExecutionSiteUnknown
    }
}


// Extension generation with lite runtime is not supported
impl crate::RpcMessage for NoResponse {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
