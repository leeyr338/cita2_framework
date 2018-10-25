// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct NewStatus {
    // message fields
    pub height: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewStatus {}

impl NewStatus {
    pub fn new() -> NewStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewStatus {
        static mut instance: ::protobuf::lazy::Lazy<NewStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewStatus,
        };
        unsafe {
            instance.get(NewStatus::new)
        }
    }

    // uint64 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }
}

impl ::protobuf::Message for NewStatus {
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
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NewStatus {
    fn new() -> NewStatus {
        NewStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    NewStatus::get_height_for_reflect,
                    NewStatus::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewStatus>(
                    "NewStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewStatus {
    fn clear(&mut self) {
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsensusMessage {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsensusMessage {}

impl ConsensusMessage {
    pub fn new() -> ConsensusMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsensusMessage {
        static mut instance: ::protobuf::lazy::Lazy<ConsensusMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsensusMessage,
        };
        unsafe {
            instance.get(ConsensusMessage::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for ConsensusMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConsensusMessage {
    fn new() -> ConsensusMessage {
        ConsensusMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsensusMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ConsensusMessage::get_data_for_reflect,
                    ConsensusMessage::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsensusMessage>(
                    "ConsensusMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsensusMessage {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsensusMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsensusMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fconsensus.proto\x1a\x10blockchain.proto\x1a\x0ccommon.proto\"#\n\t\
    NewStatus\x12\x16\n\x06height\x18\x01\x20\x01(\x04R\x06height\"&\n\x10Co\
    nsensusMessage\x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data2\x8f\x01\
    \n\x10ConsensusService\x12\x1f\n\x0bVerifyProof\x12\x06.Proof\x1a\x08.Rp\
    cBool\x125\n\x14ProcConsensusMessage\x12\x11.ConsensusMessage\x1a\n.RpcS\
    tatus\x12#\n\tSetStatus\x12\n.NewStatus\x1a\n.RpcStatusJ\xef\x03\n\x06\
    \x12\x04\x01\0\x13\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x03\x07\x19\n\t\n\x02\x03\x01\x12\x03\x04\x07\x15\nX\n\x02\x04\
    \0\x12\x04\x07\0\t\x01\x1aL/\x20FIXME:\x20This\x20two\x20message\x20is\
    \x20duplicate\x20with\x20the\x20definition\x20in\x20network.proto\n\n\n\
    \n\x03\x04\0\x01\x12\x03\x07\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\
    \x04\x16\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x08\x04\x07\x13\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x08\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08\x14\x15\n\n\n\x02\
    \x04\x01\x12\x04\x0b\0\r\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0b\x08\x18\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x0c\x04\x13\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x0c\x04\x0b\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0c\x04\t\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0c\n\x0e\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x0c\x11\x12\n\n\n\x02\x06\0\x12\x04\x0f\0\x13\x01\n\n\n\
    \x03\x06\0\x01\x12\x03\x0f\x08\x18\n\x0b\n\x04\x06\0\x02\0\x12\x03\x10\
    \x04.\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\x08\x13\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03\x10\x15\x1a\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x10%,\
    \n\x0b\n\x04\x06\0\x02\x01\x12\x03\x11\x04D\n\x0c\n\x05\x06\0\x02\x01\
    \x01\x12\x03\x11\x08\x1c\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x11\x1e.\
    \n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x119B\n\x0b\n\x04\x06\0\x02\x02\
    \x12\x03\x12\x042\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x12\x08\x11\n\
    \x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x12\x13\x1c\n\x0c\n\x05\x06\0\x02\
    \x02\x03\x12\x03\x12'0b\x06proto3\
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
