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
pub struct AddUnverifiedTransactionRet {
    // message fields
    pub ret: AddUnverifiedTransactionRetType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddUnverifiedTransactionRet {}

impl AddUnverifiedTransactionRet {
    pub fn new() -> AddUnverifiedTransactionRet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddUnverifiedTransactionRet {
        static mut instance: ::protobuf::lazy::Lazy<AddUnverifiedTransactionRet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddUnverifiedTransactionRet,
        };
        unsafe {
            instance.get(AddUnverifiedTransactionRet::new)
        }
    }

    // .AddUnverifiedTransactionRetType ret = 1;

    pub fn clear_ret(&mut self) {
        self.ret = AddUnverifiedTransactionRetType::OK;
    }

    // Param is passed by value, moved
    pub fn set_ret(&mut self, v: AddUnverifiedTransactionRetType) {
        self.ret = v;
    }

    pub fn get_ret(&self) -> AddUnverifiedTransactionRetType {
        self.ret
    }

    fn get_ret_for_reflect(&self) -> &AddUnverifiedTransactionRetType {
        &self.ret
    }

    fn mut_ret_for_reflect(&mut self) -> &mut AddUnverifiedTransactionRetType {
        &mut self.ret
    }
}

impl ::protobuf::Message for AddUnverifiedTransactionRet {
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
                    let tmp = is.read_enum()?;
                    self.ret = tmp;
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
        if self.ret != AddUnverifiedTransactionRetType::OK {
            my_size += ::protobuf::rt::enum_size(1, self.ret);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ret != AddUnverifiedTransactionRetType::OK {
            os.write_enum(1, self.ret.value())?;
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

impl ::protobuf::MessageStatic for AddUnverifiedTransactionRet {
    fn new() -> AddUnverifiedTransactionRet {
        AddUnverifiedTransactionRet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddUnverifiedTransactionRet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AddUnverifiedTransactionRetType>>(
                    "ret",
                    AddUnverifiedTransactionRet::get_ret_for_reflect,
                    AddUnverifiedTransactionRet::mut_ret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddUnverifiedTransactionRet>(
                    "AddUnverifiedTransactionRet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddUnverifiedTransactionRet {
    fn clear(&mut self) {
        self.clear_ret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddUnverifiedTransactionRet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUnverifiedTransactionRet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnverifiedTransactions {
    // message fields
    pub utxs: ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnverifiedTransactions {}

impl UnverifiedTransactions {
    pub fn new() -> UnverifiedTransactions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnverifiedTransactions {
        static mut instance: ::protobuf::lazy::Lazy<UnverifiedTransactions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnverifiedTransactions,
        };
        unsafe {
            instance.get(UnverifiedTransactions::new)
        }
    }

    // repeated .UnverifiedTransaction utxs = 1;

    pub fn clear_utxs(&mut self) {
        self.utxs.clear();
    }

    // Param is passed by value, moved
    pub fn set_utxs(&mut self, v: ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction>) {
        self.utxs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_utxs(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        &mut self.utxs
    }

    // Take field
    pub fn take_utxs(&mut self) -> ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        ::std::mem::replace(&mut self.utxs, ::protobuf::RepeatedField::new())
    }

    pub fn get_utxs(&self) -> &[super::blockchain::UnverifiedTransaction] {
        &self.utxs
    }

    fn get_utxs_for_reflect(&self) -> &::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        &self.utxs
    }

    fn mut_utxs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::UnverifiedTransaction> {
        &mut self.utxs
    }
}

impl ::protobuf::Message for UnverifiedTransactions {
    fn is_initialized(&self) -> bool {
        for v in &self.utxs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.utxs)?;
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
        for value in &self.utxs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.utxs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for UnverifiedTransactions {
    fn new() -> UnverifiedTransactions {
        UnverifiedTransactions::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnverifiedTransactions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::UnverifiedTransaction>>(
                    "utxs",
                    UnverifiedTransactions::get_utxs_for_reflect,
                    UnverifiedTransactions::mut_utxs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnverifiedTransactions>(
                    "UnverifiedTransactions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnverifiedTransactions {
    fn clear(&mut self) {
        self.clear_utxs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnverifiedTransactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnverifiedTransactions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnverifiedTransactionHashes {
    // message fields
    pub utx_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnverifiedTransactionHashes {}

impl UnverifiedTransactionHashes {
    pub fn new() -> UnverifiedTransactionHashes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnverifiedTransactionHashes {
        static mut instance: ::protobuf::lazy::Lazy<UnverifiedTransactionHashes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnverifiedTransactionHashes,
        };
        unsafe {
            instance.get(UnverifiedTransactionHashes::new)
        }
    }

    // repeated bytes utx_hashes = 1;

    pub fn clear_utx_hashes(&mut self) {
        self.utx_hashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_utx_hashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.utx_hashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_utx_hashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.utx_hashes
    }

    // Take field
    pub fn take_utx_hashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.utx_hashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_utx_hashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.utx_hashes
    }

    fn get_utx_hashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.utx_hashes
    }

    fn mut_utx_hashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.utx_hashes
    }
}

impl ::protobuf::Message for UnverifiedTransactionHashes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.utx_hashes)?;
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
        for value in &self.utx_hashes {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.utx_hashes {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for UnverifiedTransactionHashes {
    fn new() -> UnverifiedTransactionHashes {
        UnverifiedTransactionHashes::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnverifiedTransactionHashes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hashes",
                    UnverifiedTransactionHashes::get_utx_hashes_for_reflect,
                    UnverifiedTransactionHashes::mut_utx_hashes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnverifiedTransactionHashes>(
                    "UnverifiedTransactionHashes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnverifiedTransactionHashes {
    fn clear(&mut self) {
        self.clear_utx_hashes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnverifiedTransactionHashes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnverifiedTransactionHashes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckUnverifiedTransactionReq {
    // message fields
    pub node_id: u32,
    pub utx_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckUnverifiedTransactionReq {}

impl CheckUnverifiedTransactionReq {
    pub fn new() -> CheckUnverifiedTransactionReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckUnverifiedTransactionReq {
        static mut instance: ::protobuf::lazy::Lazy<CheckUnverifiedTransactionReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckUnverifiedTransactionReq,
        };
        unsafe {
            instance.get(CheckUnverifiedTransactionReq::new)
        }
    }

    // uint32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: u32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &u32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.node_id
    }

    // repeated bytes utx_hashes = 2;

    pub fn clear_utx_hashes(&mut self) {
        self.utx_hashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_utx_hashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.utx_hashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_utx_hashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.utx_hashes
    }

    // Take field
    pub fn take_utx_hashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.utx_hashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_utx_hashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.utx_hashes
    }

    fn get_utx_hashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.utx_hashes
    }

    fn mut_utx_hashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.utx_hashes
    }
}

impl ::protobuf::Message for CheckUnverifiedTransactionReq {
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
                    let tmp = is.read_uint32()?;
                    self.node_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.utx_hashes)?;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.utx_hashes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_uint32(1, self.node_id)?;
        }
        for v in &self.utx_hashes {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for CheckUnverifiedTransactionReq {
    fn new() -> CheckUnverifiedTransactionReq {
        CheckUnverifiedTransactionReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckUnverifiedTransactionReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_id",
                    CheckUnverifiedTransactionReq::get_node_id_for_reflect,
                    CheckUnverifiedTransactionReq::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hashes",
                    CheckUnverifiedTransactionReq::get_utx_hashes_for_reflect,
                    CheckUnverifiedTransactionReq::mut_utx_hashes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckUnverifiedTransactionReq>(
                    "CheckUnverifiedTransactionReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckUnverifiedTransactionReq {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_utx_hashes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckUnverifiedTransactionReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckUnverifiedTransactionReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AddUnverifiedTransactionRetType {
    OK = 0,
    InvalidNonce = 1,
    Dup = 2,
    InvalidUntilBlock = 3,
    BadSig = 4,
    NotReady = 5,
    Busy = 6,
    BadChainId = 7,
    QuotaNotEnough = 8,
    Forbidden = 9,
    InvalidValue = 10,
    InvalidVersion = 11,
    InvalidTo = 12,
}

impl ::protobuf::ProtobufEnum for AddUnverifiedTransactionRetType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AddUnverifiedTransactionRetType> {
        match value {
            0 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::OK),
            1 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::InvalidNonce),
            2 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::Dup),
            3 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::InvalidUntilBlock),
            4 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::BadSig),
            5 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::NotReady),
            6 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::Busy),
            7 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::BadChainId),
            8 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::QuotaNotEnough),
            9 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::Forbidden),
            10 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::InvalidValue),
            11 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::InvalidVersion),
            12 => ::std::option::Option::Some(AddUnverifiedTransactionRetType::InvalidTo),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AddUnverifiedTransactionRetType] = &[
            AddUnverifiedTransactionRetType::OK,
            AddUnverifiedTransactionRetType::InvalidNonce,
            AddUnverifiedTransactionRetType::Dup,
            AddUnverifiedTransactionRetType::InvalidUntilBlock,
            AddUnverifiedTransactionRetType::BadSig,
            AddUnverifiedTransactionRetType::NotReady,
            AddUnverifiedTransactionRetType::Busy,
            AddUnverifiedTransactionRetType::BadChainId,
            AddUnverifiedTransactionRetType::QuotaNotEnough,
            AddUnverifiedTransactionRetType::Forbidden,
            AddUnverifiedTransactionRetType::InvalidValue,
            AddUnverifiedTransactionRetType::InvalidVersion,
            AddUnverifiedTransactionRetType::InvalidTo,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AddUnverifiedTransactionRetType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AddUnverifiedTransactionRetType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AddUnverifiedTransactionRetType {
}

impl ::std::default::Default for AddUnverifiedTransactionRetType {
    fn default() -> Self {
        AddUnverifiedTransactionRetType::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUnverifiedTransactionRetType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\npool.proto\x1a\x10blockchain.proto\x1a\x0ccommon.proto\"Q\n\x1bAddUn\
    verifiedTransactionRet\x122\n\x03ret\x18\x01\x20\x01(\x0e2\x20.AddUnveri\
    fiedTransactionRetTypeR\x03ret\"D\n\x16UnverifiedTransactions\x12*\n\x04\
    utxs\x18\x01\x20\x03(\x0b2\x16.UnverifiedTransactionR\x04utxs\"<\n\x1bUn\
    verifiedTransactionHashes\x12\x1d\n\nutx_hashes\x18\x01\x20\x03(\x0cR\tu\
    txHashes\"W\n\x1dCheckUnverifiedTransactionReq\x12\x17\n\x07node_id\x18\
    \x01\x20\x01(\rR\x06nodeId\x12\x1d\n\nutx_hashes\x18\x02\x20\x03(\x0cR\t\
    utxHashes*\xe7\x01\n\x1fAddUnverifiedTransactionRetType\x12\x06\n\x02OK\
    \x10\0\x12\x10\n\x0cInvalidNonce\x10\x01\x12\x07\n\x03Dup\x10\x02\x12\
    \x15\n\x11InvalidUntilBlock\x10\x03\x12\n\n\x06BadSig\x10\x04\x12\x0c\n\
    \x08NotReady\x10\x05\x12\x08\n\x04Busy\x10\x06\x12\x0e\n\nBadChainId\x10\
    \x07\x12\x12\n\x0eQuotaNotEnough\x10\x08\x12\r\n\tForbidden\x10\t\x12\
    \x10\n\x0cInvalidValue\x10\n\x12\x12\n\x0eInvalidVersion\x10\x0b\x12\r\n\
    \tInvalidTo\x10\x0c2\x90\x03\n\x0bPoolService\x12P\n\x18AddUnverifiedTra\
    nsaction\x12\x16.UnverifiedTransaction\x1a\x1c.AddUnverifiedTransactionR\
    et\x12E\n\x1eAddBatchUnverifiedTransactions\x12\x17.UnverifiedTransactio\
    ns\x1a\n.RpcStatus\x12Q\n\x18GetUnverifiedTransaction\x12\x1c.Unverified\
    TransactionHashes\x1a\x17.UnverifiedTransactions\x12M\n#ProposalUnverifi\
    edTransactionHashes\x12\x08.RpcNone\x1a\x1c.UnverifiedTransactionHashes\
    \x12F\n\x1aCheckUnverifiedTransaction\x12\x1e.CheckUnverifiedTransaction\
    Req\x1a\x08.RpcBoolJ\xbb\n\n\x06\x12\x04\0\0,\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x19\n\t\n\x02\x03\x01\x12\
    \x03\x03\x07\x15\n\n\n\x02\x05\0\x12\x04\x05\0\x13\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x05\x05$\n\x0b\n\x04\x05\0\x02\0\x12\x03\x06\x04\x1b\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\x06\x04\x06\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03\x06\x19\x1a\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x07\x04\x1b\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03\x07\x04\x10\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03\x07\x19\x1a\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x08\x04\x1b\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x08\x04\x07\n\x0c\n\x05\x05\0\x02\
    \x02\x02\x12\x03\x08\x19\x1a\n\x0b\n\x04\x05\0\x02\x03\x12\x03\t\x04\x1b\
    \n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\t\x04\x15\n\x0c\n\x05\x05\0\x02\
    \x03\x02\x12\x03\t\x19\x1a\n\x0b\n\x04\x05\0\x02\x04\x12\x03\n\x04\x1b\n\
    \x0c\n\x05\x05\0\x02\x04\x01\x12\x03\n\x04\n\n\x0c\n\x05\x05\0\x02\x04\
    \x02\x12\x03\n\x19\x1a\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0b\x04\x1b\n\
    \x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x05\0\x02\
    \x05\x02\x12\x03\x0b\x19\x1a\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0c\x04\
    \x1b\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\x0c\x04\x08\n\x0c\n\x05\x05\0\
    \x02\x06\x02\x12\x03\x0c\x19\x1a\n\x0b\n\x04\x05\0\x02\x07\x12\x03\r\x04\
    \x1b\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\r\x04\x0e\n\x0c\n\x05\x05\0\
    \x02\x07\x02\x12\x03\r\x19\x1a\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x0e\x04\
    \x1b\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x0e\x04\x12\n\x0c\n\x05\x05\0\
    \x02\x08\x02\x12\x03\x0e\x19\x1a\n\x0b\n\x04\x05\0\x02\t\x12\x03\x0f\x04\
    \x1b\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x0f\x04\r\n\x0c\n\x05\x05\0\x02\
    \t\x02\x12\x03\x0f\x19\x1a\n\x0b\n\x04\x05\0\x02\n\x12\x03\x10\x04\x1c\n\
    \x0c\n\x05\x05\0\x02\n\x01\x12\x03\x10\x04\x10\n\x0c\n\x05\x05\0\x02\n\
    \x02\x12\x03\x10\x19\x1b\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x11\x04\x1c\n\
    \x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x11\x04\x12\n\x0c\n\x05\x05\0\x02\
    \x0b\x02\x12\x03\x11\x19\x1b\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x12\x04\
    \x1c\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x12\x04\r\n\x0c\n\x05\x05\0\
    \x02\x0c\x02\x12\x03\x12\x19\x1b\n\n\n\x02\x04\0\x12\x04\x15\0\x17\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x15\x08#\n\x0b\n\x04\x04\0\x02\0\x12\x03\x16\
    \x04:\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x16\x04\x15%\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03\x16\x04#\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x16),\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1689\n\n\n\x02\x04\x01\x12\x04\x19\0\
    \x1b\x01\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\x1e\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x1a\x04:\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x1a\x04\
    \x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x1a\r\"\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x1a)-\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1a89\n\n\
    \n\x02\x04\x02\x12\x04\x1d\0\x1f\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1d\
    \x08#\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\x04:\n\x0c\n\x05\x04\x02\x02\
    \0\x04\x12\x03\x1e\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x1e\r\
    \x12\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1e)3\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03\x1e89\n\n\n\x02\x04\x03\x12\x04!\0$\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03!\x08%\n\x0b\n\x04\x04\x03\x02\0\x12\x03\"\x04:\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04\"\x04!'\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\
    \"\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\")0\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03\"89\n\x0b\n\x04\x04\x03\x02\x01\x12\x03#\x04:\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x03#\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\
    \x05\x12\x03#\r\x12\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03#)3\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x03#89\n\n\n\x02\x06\0\x12\x04&\0,\x01\n\n\
    \n\x03\x06\0\x01\x12\x03&\x08\x13\n\x0b\n\x04\x06\0\x02\0\x12\x03'\x04_\
    \n\x0c\n\x05\x06\0\x02\0\x01\x12\x03'\x08\x20\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03'\"7\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03'B]\n\x0b\n\x04\x06\
    \0\x02\x01\x12\x03(\x04T\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03(\x08&\n\
    \x0c\n\x05\x06\0\x02\x01\x02\x12\x03((>\n\x0c\n\x05\x06\0\x02\x01\x03\
    \x12\x03(IR\n\x0b\n\x04\x06\0\x02\x02\x12\x03)\x04`\n\x0c\n\x05\x06\0\
    \x02\x02\x01\x12\x03)\x08\x20\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03)\"=\
    \n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03)H^\n\x0b\n\x04\x06\0\x02\x03\x12\
    \x03*\x04\\\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03*\x08+\n\x0c\n\x05\x06\
    \0\x02\x03\x02\x12\x03*-4\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03*?Z\n\x0b\
    \n\x04\x06\0\x02\x04\x12\x03+\x04U\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03\
    +\x08\"\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03+$A\n\x0c\n\x05\x06\0\x02\
    \x04\x03\x12\x03+LSb\x06proto3\
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
