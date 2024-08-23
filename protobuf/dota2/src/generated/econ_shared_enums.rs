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

//! Generated file from `econ_shared_enums.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CMsgGenericResult)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgGenericResult {
    // message fields
    // @@protoc_insertion_point(field:CMsgGenericResult.eresult)
    pub eresult: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CMsgGenericResult.debug_message)
    pub debug_message: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgGenericResult.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgGenericResult {
    fn default() -> &'a CMsgGenericResult {
        <CMsgGenericResult as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CMsgGenericResult {
    pub fn new() -> CMsgGenericResult {
        ::std::default::Default::default()
    }

    // optional uint32 eresult = 1;

    pub fn eresult(&self) -> u32 {
        self.eresult.unwrap_or(2u32)
    }

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    // optional string debug_message = 2;

    pub fn debug_message(&self) -> &str {
        match self.debug_message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_debug_message(&mut self) {
        self.debug_message = ::std::option::Option::None;
    }

    pub fn has_debug_message(&self) -> bool {
        self.debug_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_message(&mut self, v: ::std::string::String) {
        self.debug_message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_message(&mut self) -> &mut ::std::string::String {
        if self.debug_message.is_none() {
            self.debug_message = ::std::option::Option::Some(::std::string::String::new());
        }
        self.debug_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_message(&mut self) -> ::std::string::String {
        self.debug_message.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CMsgGenericResult {
    const NAME: &'static str = "CMsgGenericResult";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.eresult = ::std::option::Option::Some(is.read_uint32()?);
                },
                18 => {
                    self.debug_message = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.debug_message.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::string_size(2, &v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.eresult {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.debug_message.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CMsgGenericResult {
        CMsgGenericResult::new()
    }

    fn clear(&mut self) {
        self.eresult = ::std::option::Option::None;
        self.debug_message = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgGenericResult {
        static instance: CMsgGenericResult = CMsgGenericResult {
            eresult: ::std::option::Option::None,
            debug_message: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EGCEconBaseMsg)
pub enum EGCEconBaseMsg {
    // @@protoc_insertion_point(enum_value:EGCEconBaseMsg.k_EMsgGCGenericResult)
    k_EMsgGCGenericResult = 2579,
}

impl ::steam_vent_proto_common::protobuf::Enum for EGCEconBaseMsg {
    const NAME: &'static str = "EGCEconBaseMsg";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCEconBaseMsg> {
        match value {
            2579 => ::std::option::Option::Some(EGCEconBaseMsg::k_EMsgGCGenericResult),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EGCEconBaseMsg> {
        match str {
            "k_EMsgGCGenericResult" => ::std::option::Option::Some(EGCEconBaseMsg::k_EMsgGCGenericResult),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EGCEconBaseMsg] = &[
        EGCEconBaseMsg::k_EMsgGCGenericResult,
    ];
}

// Note, `Default` is implemented although default value is not 0
impl ::std::default::Default for EGCEconBaseMsg {
    fn default() -> Self {
        EGCEconBaseMsg::k_EMsgGCGenericResult
    }
}


#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EGCMsgResponse)
pub enum EGCMsgResponse {
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseOK)
    k_EGCMsgResponseOK = 0,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseDenied)
    k_EGCMsgResponseDenied = 1,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseServerError)
    k_EGCMsgResponseServerError = 2,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseTimeout)
    k_EGCMsgResponseTimeout = 3,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseInvalid)
    k_EGCMsgResponseInvalid = 4,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseNoMatch)
    k_EGCMsgResponseNoMatch = 5,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseUnknownError)
    k_EGCMsgResponseUnknownError = 6,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgResponseNotLoggedOn)
    k_EGCMsgResponseNotLoggedOn = 7,
    // @@protoc_insertion_point(enum_value:EGCMsgResponse.k_EGCMsgFailedToCreate)
    k_EGCMsgFailedToCreate = 8,
}

impl ::steam_vent_proto_common::protobuf::Enum for EGCMsgResponse {
    const NAME: &'static str = "EGCMsgResponse";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCMsgResponse> {
        match value {
            0 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseOK),
            1 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseDenied),
            2 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseServerError),
            3 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseTimeout),
            4 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseInvalid),
            5 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNoMatch),
            6 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseUnknownError),
            7 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNotLoggedOn),
            8 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgFailedToCreate),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EGCMsgResponse> {
        match str {
            "k_EGCMsgResponseOK" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseOK),
            "k_EGCMsgResponseDenied" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseDenied),
            "k_EGCMsgResponseServerError" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseServerError),
            "k_EGCMsgResponseTimeout" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseTimeout),
            "k_EGCMsgResponseInvalid" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseInvalid),
            "k_EGCMsgResponseNoMatch" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNoMatch),
            "k_EGCMsgResponseUnknownError" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseUnknownError),
            "k_EGCMsgResponseNotLoggedOn" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNotLoggedOn),
            "k_EGCMsgFailedToCreate" => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgFailedToCreate),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EGCMsgResponse] = &[
        EGCMsgResponse::k_EGCMsgResponseOK,
        EGCMsgResponse::k_EGCMsgResponseDenied,
        EGCMsgResponse::k_EGCMsgResponseServerError,
        EGCMsgResponse::k_EGCMsgResponseTimeout,
        EGCMsgResponse::k_EGCMsgResponseInvalid,
        EGCMsgResponse::k_EGCMsgResponseNoMatch,
        EGCMsgResponse::k_EGCMsgResponseUnknownError,
        EGCMsgResponse::k_EGCMsgResponseNotLoggedOn,
        EGCMsgResponse::k_EGCMsgFailedToCreate,
    ];
}

impl ::std::default::Default for EGCMsgResponse {
    fn default() -> Self {
        EGCMsgResponse::k_EGCMsgResponseOK
    }
}


#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EGCMsgUseItemResponse)
pub enum EGCMsgUseItemResponse {
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_ItemUsed)
    k_EGCMsgUseItemResponse_ItemUsed = 0,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_GiftNoOtherPlayers)
    k_EGCMsgUseItemResponse_GiftNoOtherPlayers = 1,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_ServerError)
    k_EGCMsgUseItemResponse_ServerError = 2,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_MiniGameAlreadyStarted)
    k_EGCMsgUseItemResponse_MiniGameAlreadyStarted = 3,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted)
    k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted = 4,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted)
    k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted = 5,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_NotInLowPriorityPool)
    k_EGCMsgUseItemResponse_NotInLowPriorityPool = 6,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_NotHighEnoughLevel)
    k_EGCMsgUseItemResponse_NotHighEnoughLevel = 7,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_EventNotActive)
    k_EGCMsgUseItemResponse_EventNotActive = 8,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted)
    k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted = 9,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_MissingRequirement)
    k_EGCMsgUseItemResponse_MissingRequirement = 10,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew)
    k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew = 11,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_EmoticonUnlock_Complete)
    k_EGCMsgUseItemResponse_EmoticonUnlock_Complete = 12,
    // @@protoc_insertion_point(enum_value:EGCMsgUseItemResponse.k_EGCMsgUseItemResponse_ItemUsed_Compendium)
    k_EGCMsgUseItemResponse_ItemUsed_Compendium = 13,
}

impl ::steam_vent_proto_common::protobuf::Enum for EGCMsgUseItemResponse {
    const NAME: &'static str = "EGCMsgUseItemResponse";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCMsgUseItemResponse> {
        match value {
            0 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed),
            1 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_GiftNoOtherPlayers),
            2 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ServerError),
            3 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MiniGameAlreadyStarted),
            4 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted),
            5 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted),
            6 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotInLowPriorityPool),
            7 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotHighEnoughLevel),
            8 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EventNotActive),
            9 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted),
            10 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MissingRequirement),
            11 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew),
            12 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_Complete),
            13 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_Compendium),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EGCMsgUseItemResponse> {
        match str {
            "k_EGCMsgUseItemResponse_ItemUsed" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed),
            "k_EGCMsgUseItemResponse_GiftNoOtherPlayers" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_GiftNoOtherPlayers),
            "k_EGCMsgUseItemResponse_ServerError" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ServerError),
            "k_EGCMsgUseItemResponse_MiniGameAlreadyStarted" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MiniGameAlreadyStarted),
            "k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted),
            "k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted),
            "k_EGCMsgUseItemResponse_NotInLowPriorityPool" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotInLowPriorityPool),
            "k_EGCMsgUseItemResponse_NotHighEnoughLevel" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotHighEnoughLevel),
            "k_EGCMsgUseItemResponse_EventNotActive" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EventNotActive),
            "k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted),
            "k_EGCMsgUseItemResponse_MissingRequirement" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MissingRequirement),
            "k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew),
            "k_EGCMsgUseItemResponse_EmoticonUnlock_Complete" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_Complete),
            "k_EGCMsgUseItemResponse_ItemUsed_Compendium" => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_Compendium),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EGCMsgUseItemResponse] = &[
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_GiftNoOtherPlayers,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ServerError,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MiniGameAlreadyStarted,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotInLowPriorityPool,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotHighEnoughLevel,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EventNotActive,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MissingRequirement,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_Complete,
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_Compendium,
    ];
}

impl ::std::default::Default for EGCMsgUseItemResponse {
    fn default() -> Self {
        EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed
    }
}



const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

impl ::steam_vent_proto_common::RpcMessage for CMsgGenericResult {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessageWithKind for CMsgGenericResult {
    type KindEnum = crate::econ_shared_enums::EGCEconBaseMsg;
    const KIND: Self::KindEnum = crate::econ_shared_enums::EGCEconBaseMsg::k_EMsgGCGenericResult;
}
impl ::steam_vent_proto_common::MsgKindEnum for EGCEconBaseMsg {}