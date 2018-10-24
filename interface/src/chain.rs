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
pub struct AddBlockRet {
    // message fields
    pub height: u64,
    pub block_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddBlockRet {}

impl AddBlockRet {
    pub fn new() -> AddBlockRet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddBlockRet {
        static mut instance: ::protobuf::lazy::Lazy<AddBlockRet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddBlockRet,
        };
        unsafe {
            instance.get(AddBlockRet::new)
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

    // bytes block_hash = 2;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_hash, ::std::vec::Vec::new())
    }

    pub fn get_block_hash(&self) -> &[u8] {
        &self.block_hash
    }

    fn get_block_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block_hash
    }

    fn mut_block_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }
}

impl ::protobuf::Message for AddBlockRet {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
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
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.block_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        if !self.block_hash.is_empty() {
            os.write_bytes(2, &self.block_hash)?;
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

impl ::protobuf::MessageStatic for AddBlockRet {
    fn new() -> AddBlockRet {
        AddBlockRet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddBlockRet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    AddBlockRet::get_height_for_reflect,
                    AddBlockRet::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hash",
                    AddBlockRet::get_block_hash_for_reflect,
                    AddBlockRet::mut_block_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddBlockRet>(
                    "AddBlockRet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddBlockRet {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_block_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddBlockRet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddBlockRet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockReq {
    // message fields
    pub block_height: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockReq {}

impl GetBlockReq {
    pub fn new() -> GetBlockReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockReq {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockReq,
        };
        unsafe {
            instance.get(GetBlockReq::new)
        }
    }

    // bytes block_height = 1;

    pub fn clear_block_height(&mut self) {
        self.block_height.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_height(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_height = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_height(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_height
    }

    // Take field
    pub fn take_block_height(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_height, ::std::vec::Vec::new())
    }

    pub fn get_block_height(&self) -> &[u8] {
        &self.block_height
    }

    fn get_block_height_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block_height
    }

    fn mut_block_height_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_height
    }
}

impl ::protobuf::Message for GetBlockReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_height)?;
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
        if !self.block_height.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block_height);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block_height.is_empty() {
            os.write_bytes(1, &self.block_height)?;
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

impl ::protobuf::MessageStatic for GetBlockReq {
    fn new() -> GetBlockReq {
        GetBlockReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_height",
                    GetBlockReq::get_block_height_for_reflect,
                    GetBlockReq::mut_block_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockReq>(
                    "GetBlockReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockReq {
    fn clear(&mut self) {
        self.clear_block_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReceiptReq {
    // message fields
    pub utx_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReceiptReq {}

impl GetReceiptReq {
    pub fn new() -> GetReceiptReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReceiptReq {
        static mut instance: ::protobuf::lazy::Lazy<GetReceiptReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReceiptReq,
        };
        unsafe {
            instance.get(GetReceiptReq::new)
        }
    }

    // bytes utx_hash = 1;

    pub fn clear_utx_hash(&mut self) {
        self.utx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_utx_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.utx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_utx_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.utx_hash
    }

    // Take field
    pub fn take_utx_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.utx_hash, ::std::vec::Vec::new())
    }

    pub fn get_utx_hash(&self) -> &[u8] {
        &self.utx_hash
    }

    fn get_utx_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.utx_hash
    }

    fn mut_utx_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.utx_hash
    }
}

impl ::protobuf::Message for GetReceiptReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.utx_hash)?;
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
        if !self.utx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.utx_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.utx_hash.is_empty() {
            os.write_bytes(1, &self.utx_hash)?;
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

impl ::protobuf::MessageStatic for GetReceiptReq {
    fn new() -> GetReceiptReq {
        GetReceiptReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReceiptReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hash",
                    GetReceiptReq::get_utx_hash_for_reflect,
                    GetReceiptReq::mut_utx_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReceiptReq>(
                    "GetReceiptReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReceiptReq {
    fn clear(&mut self) {
        self.clear_utx_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReceiptReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReceiptReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTransactionReq {
    // message fields
    pub utx_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTransactionReq {}

impl GetTransactionReq {
    pub fn new() -> GetTransactionReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTransactionReq {
        static mut instance: ::protobuf::lazy::Lazy<GetTransactionReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTransactionReq,
        };
        unsafe {
            instance.get(GetTransactionReq::new)
        }
    }

    // bytes utx_hash = 1;

    pub fn clear_utx_hash(&mut self) {
        self.utx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_utx_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.utx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_utx_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.utx_hash
    }

    // Take field
    pub fn take_utx_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.utx_hash, ::std::vec::Vec::new())
    }

    pub fn get_utx_hash(&self) -> &[u8] {
        &self.utx_hash
    }

    fn get_utx_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.utx_hash
    }

    fn mut_utx_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.utx_hash
    }
}

impl ::protobuf::Message for GetTransactionReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.utx_hash)?;
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
        if !self.utx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.utx_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.utx_hash.is_empty() {
            os.write_bytes(1, &self.utx_hash)?;
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

impl ::protobuf::MessageStatic for GetTransactionReq {
    fn new() -> GetTransactionReq {
        GetTransactionReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTransactionReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hash",
                    GetTransactionReq::get_utx_hash_for_reflect,
                    GetTransactionReq::mut_utx_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTransactionReq>(
                    "GetTransactionReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTransactionReq {
    fn clear(&mut self) {
        self.clear_utx_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTransactionReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTransactionReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bchain.proto\x1a\x10blockchain.proto\"D\n\x0bAddBlockRet\x12\x16\n\
    \x06height\x18\x01\x20\x01(\x04R\x06height\x12\x1d\n\nblock_hash\x18\x02\
    \x20\x01(\x0cR\tblockHash\"0\n\x0bGetBlockReq\x12!\n\x0cblock_height\x18\
    \x01\x20\x01(\x0cR\x0bblockHeight\"*\n\rGetReceiptReq\x12\x19\n\x08utx_h\
    ash\x18\x01\x20\x01(\x0cR\x07utxHash\".\n\x11GetTransactionReq\x12\x19\n\
    \x08utx_hash\x18\x01\x20\x01(\x0cR\x07utxHash2\xb4\x01\n\x0cChainService\
    \x12\x20\n\x08AddBlock\x12\x06.Block\x1a\x0c.AddBlockRet\x12\x20\n\x08Ge\
    tBlock\x12\x0c.GetBlockReq\x1a\x06.Block\x12&\n\nGetReceipt\x12\x0e.GetR\
    eceiptReq\x1a\x08.Receipt\x128\n\x0eGetTransaction\x12\x12.GetTransactio\
    nReq\x1a\x12.SignedTransactionJ\xcf\x05\n\x06\x12\x04\x01\0\x1b\x01\n\
    \x08\n\x01\x0c\x12\x03\x01\0\x12\n\t\n\x02\x03\0\x12\x03\x03\x07\x19\n\n\
    \n\x02\x04\0\x12\x04\x05\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\
    \x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04'\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\x06\x04\x05\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x04\n\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x14\x1a\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x06%&\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x04'\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x07\x04\x06'\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x07\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x14\x1e\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07%&\n\n\n\x02\x04\x01\x12\x04\n\0\
    \x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x13\n\x0b\n\x04\x04\x01\x02\
    \0\x12\x03\x0b\x04'\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0b\x04\n\x15\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0b\x04\t\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x0b\x14\x20\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b%&\n\n\
    \n\x02\x04\x02\x12\x04\x0e\0\x10\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\
    \x08\x15\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0f\x04'\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04\x0f\x04\x0e\x17\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\
    \x0f\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0f\x14\x1c\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03\x0f%&\n\n\n\x02\x04\x03\x12\x04\x12\0\x14\x01\
    \n\n\n\x03\x04\x03\x01\x12\x03\x12\x08\x19\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03\x13\x04'\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x13\x04\x12\x1b\n\x0c\
    \n\x05\x04\x03\x02\0\x05\x12\x03\x13\x04\t\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03\x13\x14\x1c\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x13%&\n\n\n\
    \x02\x06\0\x12\x04\x16\0\x1b\x01\n\n\n\x03\x06\0\x01\x12\x03\x16\x08\x14\
    \n\x0b\n\x04\x06\0\x02\0\x12\x03\x17\x04/\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\x17\x08\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x17\x12\x17\n\
    \x0c\n\x05\x06\0\x02\0\x03\x12\x03\x17\"-\n\x0b\n\x04\x06\0\x02\x01\x12\
    \x03\x18\x04/\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x18\x08\x10\n\x0c\n\
    \x05\x06\0\x02\x01\x02\x12\x03\x18\x12\x1d\n\x0c\n\x05\x06\0\x02\x01\x03\
    \x12\x03\x18(-\n\x0b\n\x04\x06\0\x02\x02\x12\x03\x19\x045\n\x0c\n\x05\
    \x06\0\x02\x02\x01\x12\x03\x19\x08\x12\n\x0c\n\x05\x06\0\x02\x02\x02\x12\
    \x03\x19\x14!\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x19,3\n\x0b\n\x04\
    \x06\0\x02\x03\x12\x03\x1a\x04G\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\
    \x1a\x08\x16\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x1a\x18)\n\x0c\n\x05\
    \x06\0\x02\x03\x03\x12\x03\x1a4Eb\x06proto3\
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
