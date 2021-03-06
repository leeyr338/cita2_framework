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
pub struct ApplyReq {
    // message fields
    pub stx: ::protobuf::SingularPtrField<super::blockchain::SignedTransaction>,
    pub db_ref: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApplyReq {}

impl ApplyReq {
    pub fn new() -> ApplyReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApplyReq {
        static mut instance: ::protobuf::lazy::Lazy<ApplyReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApplyReq,
        };
        unsafe {
            instance.get(ApplyReq::new)
        }
    }

    // .SignedTransaction stx = 1;

    pub fn clear_stx(&mut self) {
        self.stx.clear();
    }

    pub fn has_stx(&self) -> bool {
        self.stx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stx(&mut self, v: super::blockchain::SignedTransaction) {
        self.stx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stx(&mut self) -> &mut super::blockchain::SignedTransaction {
        if self.stx.is_none() {
            self.stx.set_default();
        }
        self.stx.as_mut().unwrap()
    }

    // Take field
    pub fn take_stx(&mut self) -> super::blockchain::SignedTransaction {
        self.stx.take().unwrap_or_else(|| super::blockchain::SignedTransaction::new())
    }

    pub fn get_stx(&self) -> &super::blockchain::SignedTransaction {
        self.stx.as_ref().unwrap_or_else(|| super::blockchain::SignedTransaction::default_instance())
    }

    fn get_stx_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::SignedTransaction> {
        &self.stx
    }

    fn mut_stx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::SignedTransaction> {
        &mut self.stx
    }

    // uint64 db_ref = 2;

    pub fn clear_db_ref(&mut self) {
        self.db_ref = 0;
    }

    // Param is passed by value, moved
    pub fn set_db_ref(&mut self, v: u64) {
        self.db_ref = v;
    }

    pub fn get_db_ref(&self) -> u64 {
        self.db_ref
    }

    fn get_db_ref_for_reflect(&self) -> &u64 {
        &self.db_ref
    }

    fn mut_db_ref_for_reflect(&mut self) -> &mut u64 {
        &mut self.db_ref
    }
}

impl ::protobuf::Message for ApplyReq {
    fn is_initialized(&self) -> bool {
        for v in &self.stx {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stx)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.db_ref = tmp;
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
        if let Some(ref v) = self.stx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.db_ref != 0 {
            my_size += ::protobuf::rt::value_size(2, self.db_ref, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.stx.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.db_ref != 0 {
            os.write_uint64(2, self.db_ref)?;
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

impl ::protobuf::MessageStatic for ApplyReq {
    fn new() -> ApplyReq {
        ApplyReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApplyReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::SignedTransaction>>(
                    "stx",
                    ApplyReq::get_stx_for_reflect,
                    ApplyReq::mut_stx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "db_ref",
                    ApplyReq::get_db_ref_for_reflect,
                    ApplyReq::mut_db_ref_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApplyReq>(
                    "ApplyReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApplyReq {
    fn clear(&mut self) {
        self.clear_stx();
        self.clear_db_ref();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApplyReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApplyReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApplyRet {
    // message fields
    pub receipt: ::protobuf::SingularPtrField<super::blockchain::Receipt>,
    pub receipt_err: super::blockchain::ReceiptError,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApplyRet {}

impl ApplyRet {
    pub fn new() -> ApplyRet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApplyRet {
        static mut instance: ::protobuf::lazy::Lazy<ApplyRet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApplyRet,
        };
        unsafe {
            instance.get(ApplyRet::new)
        }
    }

    // .Receipt receipt = 1;

    pub fn clear_receipt(&mut self) {
        self.receipt.clear();
    }

    pub fn has_receipt(&self) -> bool {
        self.receipt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_receipt(&mut self, v: super::blockchain::Receipt) {
        self.receipt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipt(&mut self) -> &mut super::blockchain::Receipt {
        if self.receipt.is_none() {
            self.receipt.set_default();
        }
        self.receipt.as_mut().unwrap()
    }

    // Take field
    pub fn take_receipt(&mut self) -> super::blockchain::Receipt {
        self.receipt.take().unwrap_or_else(|| super::blockchain::Receipt::new())
    }

    pub fn get_receipt(&self) -> &super::blockchain::Receipt {
        self.receipt.as_ref().unwrap_or_else(|| super::blockchain::Receipt::default_instance())
    }

    fn get_receipt_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::Receipt> {
        &self.receipt
    }

    fn mut_receipt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::Receipt> {
        &mut self.receipt
    }

    // .ReceiptError receipt_err = 2;

    pub fn clear_receipt_err(&mut self) {
        self.receipt_err = super::blockchain::ReceiptError::NotEnoughBaseQuota;
    }

    // Param is passed by value, moved
    pub fn set_receipt_err(&mut self, v: super::blockchain::ReceiptError) {
        self.receipt_err = v;
    }

    pub fn get_receipt_err(&self) -> super::blockchain::ReceiptError {
        self.receipt_err
    }

    fn get_receipt_err_for_reflect(&self) -> &super::blockchain::ReceiptError {
        &self.receipt_err
    }

    fn mut_receipt_err_for_reflect(&mut self) -> &mut super::blockchain::ReceiptError {
        &mut self.receipt_err
    }
}

impl ::protobuf::Message for ApplyRet {
    fn is_initialized(&self) -> bool {
        for v in &self.receipt {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.receipt)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.receipt_err = tmp;
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
        if let Some(ref v) = self.receipt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.receipt_err != super::blockchain::ReceiptError::NotEnoughBaseQuota {
            my_size += ::protobuf::rt::enum_size(2, self.receipt_err);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.receipt.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.receipt_err != super::blockchain::ReceiptError::NotEnoughBaseQuota {
            os.write_enum(2, self.receipt_err.value())?;
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

impl ::protobuf::MessageStatic for ApplyRet {
    fn new() -> ApplyRet {
        ApplyRet::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApplyRet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Receipt>>(
                    "receipt",
                    ApplyRet::get_receipt_for_reflect,
                    ApplyRet::mut_receipt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::blockchain::ReceiptError>>(
                    "receipt_err",
                    ApplyRet::get_receipt_err_for_reflect,
                    ApplyRet::mut_receipt_err_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApplyRet>(
                    "ApplyRet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApplyRet {
    fn clear(&mut self) {
        self.clear_receipt();
        self.clear_receipt_err();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApplyRet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApplyRet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eexecutor.proto\x1a\x10blockchain.proto\"G\n\x08ApplyReq\x12$\n\x03\
    stx\x18\x01\x20\x01(\x0b2\x12.SignedTransactionR\x03stx\x12\x15\n\x06db_\
    ref\x18\x02\x20\x01(\x04R\x05dbRef\"^\n\x08ApplyRet\x12\"\n\x07receipt\
    \x18\x01\x20\x01(\x0b2\x08.ReceiptR\x07receipt\x12.\n\x0breceipt_err\x18\
    \x02\x20\x01(\x0e2\r.ReceiptErrorR\nreceiptErr20\n\x0fExecutorService\
    \x12\x1d\n\x05Apply\x12\t.ApplyReq\x1a\t.ApplyRetJ\xb4\x03\n\x06\x12\x04\
    \x01\0\x11\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\t\n\x02\x03\0\x12\x03\
    \x03\x07\x19\n\n\n\x02\x04\0\x12\x04\x05\0\x08\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x05\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04)\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\x06\x04\x05\x12\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x06\x04\x15\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x1b\x1e\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x06'(\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\
    \x04)\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x07\x04\x06)\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\
    \x1b!\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07'(\n\n\n\x02\x04\x01\x12\
    \x04\n\0\r\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x10\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03\x0b\x04-\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0b\x04\
    \n\x12\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0b\x04\x0b\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\x0b\x1b\"\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \x0b+,\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0c\x04-\n\r\n\x05\x04\x01\
    \x02\x01\x04\x12\x04\x0c\x04\x0b-\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\
    \x03\x0c\x04\x10\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\x1b&\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\x0c+,\n\n\n\x02\x06\0\x12\x04\x0f\0\
    \x11\x01\n\n\n\x03\x06\0\x01\x12\x03\x0f\x08\x17\n\x0b\n\x04\x06\0\x02\0\
    \x12\x03\x10\x04,\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\x08\r\n\x0c\n\
    \x05\x06\0\x02\0\x02\x12\x03\x10\x0f\x17\n\x0c\n\x05\x06\0\x02\0\x03\x12\
    \x03\x10\"*b\x06proto3\
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
