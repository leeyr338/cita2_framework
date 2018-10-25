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
pub struct Transaction {
    // message fields
    pub to: ::std::vec::Vec<u8>,
    pub nonce: ::std::string::String,
    pub quota: u64,
    pub valid_until_block: u64,
    pub data: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    pub chain_id: ::std::vec::Vec<u8>,
    pub version: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // bytes to = 1;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::vec::Vec<u8>) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to, ::std::vec::Vec::new())
    }

    pub fn get_to(&self) -> &[u8] {
        &self.to
    }

    fn get_to_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to
    }

    // string nonce = 2;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // uint64 quota = 3;

    pub fn clear_quota(&mut self) {
        self.quota = 0;
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = v;
    }

    pub fn get_quota(&self) -> u64 {
        self.quota
    }

    fn get_quota_for_reflect(&self) -> &u64 {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut u64 {
        &mut self.quota
    }

    // uint64 valid_until_block = 4;

    pub fn clear_valid_until_block(&mut self) {
        self.valid_until_block = 0;
    }

    // Param is passed by value, moved
    pub fn set_valid_until_block(&mut self, v: u64) {
        self.valid_until_block = v;
    }

    pub fn get_valid_until_block(&self) -> u64 {
        self.valid_until_block
    }

    fn get_valid_until_block_for_reflect(&self) -> &u64 {
        &self.valid_until_block
    }

    fn mut_valid_until_block_for_reflect(&mut self) -> &mut u64 {
        &mut self.valid_until_block
    }

    // bytes data = 5;

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

    // bytes value = 6;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // bytes chain_id = 7;

    pub fn clear_chain_id(&mut self) {
        self.chain_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_chain_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.chain_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chain_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.chain_id
    }

    // Take field
    pub fn take_chain_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.chain_id, ::std::vec::Vec::new())
    }

    pub fn get_chain_id(&self) -> &[u8] {
        &self.chain_id
    }

    fn get_chain_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.chain_id
    }

    fn mut_chain_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.chain_id
    }

    // uint32 version = 8;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.valid_until_block = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.chain_id)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
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
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.to);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.nonce);
        }
        if self.quota != 0 {
            my_size += ::protobuf::rt::value_size(3, self.quota, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.valid_until_block != 0 {
            my_size += ::protobuf::rt::value_size(4, self.valid_until_block, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.value);
        }
        if !self.chain_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.chain_id);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(8, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.to.is_empty() {
            os.write_bytes(1, &self.to)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(2, &self.nonce)?;
        }
        if self.quota != 0 {
            os.write_uint64(3, self.quota)?;
        }
        if self.valid_until_block != 0 {
            os.write_uint64(4, self.valid_until_block)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(6, &self.value)?;
        }
        if !self.chain_id.is_empty() {
            os.write_bytes(7, &self.chain_id)?;
        }
        if self.version != 0 {
            os.write_uint32(8, self.version)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "to",
                    Transaction::get_to_for_reflect,
                    Transaction::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    Transaction::get_nonce_for_reflect,
                    Transaction::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    Transaction::get_quota_for_reflect,
                    Transaction::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "valid_until_block",
                    Transaction::get_valid_until_block_for_reflect,
                    Transaction::mut_valid_until_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Transaction::get_data_for_reflect,
                    Transaction::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Transaction::get_value_for_reflect,
                    Transaction::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "chain_id",
                    Transaction::get_chain_id_for_reflect,
                    Transaction::mut_chain_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Transaction::get_version_for_reflect,
                    Transaction::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_to();
        self.clear_nonce();
        self.clear_quota();
        self.clear_valid_until_block();
        self.clear_data();
        self.clear_value();
        self.clear_chain_id();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnverifiedTransaction {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<Transaction>,
    pub signature: ::std::vec::Vec<u8>,
    pub crypto: Crypto,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnverifiedTransaction {}

impl UnverifiedTransaction {
    pub fn new() -> UnverifiedTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnverifiedTransaction {
        static mut instance: ::protobuf::lazy::Lazy<UnverifiedTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnverifiedTransaction,
        };
        unsafe {
            instance.get(UnverifiedTransaction::new)
        }
    }

    // .Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> Transaction {
        self.transaction.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_transaction(&self) -> &Transaction {
        self.transaction.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<Transaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Transaction> {
        &mut self.transaction
    }

    // bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // .Crypto crypto = 3;

    pub fn clear_crypto(&mut self) {
        self.crypto = Crypto::SECP;
    }

    // Param is passed by value, moved
    pub fn set_crypto(&mut self, v: Crypto) {
        self.crypto = v;
    }

    pub fn get_crypto(&self) -> Crypto {
        self.crypto
    }

    fn get_crypto_for_reflect(&self) -> &Crypto {
        &self.crypto
    }

    fn mut_crypto_for_reflect(&mut self) -> &mut Crypto {
        &mut self.crypto
    }
}

impl ::protobuf::Message for UnverifiedTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crypto = tmp;
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
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        if self.crypto != Crypto::SECP {
            my_size += ::protobuf::rt::enum_size(3, self.crypto);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
        }
        if self.crypto != Crypto::SECP {
            os.write_enum(3, self.crypto.value())?;
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

impl ::protobuf::MessageStatic for UnverifiedTransaction {
    fn new() -> UnverifiedTransaction {
        UnverifiedTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnverifiedTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transaction",
                    UnverifiedTransaction::get_transaction_for_reflect,
                    UnverifiedTransaction::mut_transaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    UnverifiedTransaction::get_signature_for_reflect,
                    UnverifiedTransaction::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Crypto>>(
                    "crypto",
                    UnverifiedTransaction::get_crypto_for_reflect,
                    UnverifiedTransaction::mut_crypto_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnverifiedTransaction>(
                    "UnverifiedTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnverifiedTransaction {
    fn clear(&mut self) {
        self.clear_transaction();
        self.clear_signature();
        self.clear_crypto();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnverifiedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnverifiedTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedTransaction {
    // message fields
    pub transaction_with_sig: ::protobuf::SingularPtrField<UnverifiedTransaction>,
    pub utx_hash: ::std::vec::Vec<u8>,
    pub public: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedTransaction {}

impl SignedTransaction {
    pub fn new() -> SignedTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedTransaction {
        static mut instance: ::protobuf::lazy::Lazy<SignedTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedTransaction,
        };
        unsafe {
            instance.get(SignedTransaction::new)
        }
    }

    // .UnverifiedTransaction transaction_with_sig = 1;

    pub fn clear_transaction_with_sig(&mut self) {
        self.transaction_with_sig.clear();
    }

    pub fn has_transaction_with_sig(&self) -> bool {
        self.transaction_with_sig.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_with_sig(&mut self, v: UnverifiedTransaction) {
        self.transaction_with_sig = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_with_sig(&mut self) -> &mut UnverifiedTransaction {
        if self.transaction_with_sig.is_none() {
            self.transaction_with_sig.set_default();
        }
        self.transaction_with_sig.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction_with_sig(&mut self) -> UnverifiedTransaction {
        self.transaction_with_sig.take().unwrap_or_else(|| UnverifiedTransaction::new())
    }

    pub fn get_transaction_with_sig(&self) -> &UnverifiedTransaction {
        self.transaction_with_sig.as_ref().unwrap_or_else(|| UnverifiedTransaction::default_instance())
    }

    fn get_transaction_with_sig_for_reflect(&self) -> &::protobuf::SingularPtrField<UnverifiedTransaction> {
        &self.transaction_with_sig
    }

    fn mut_transaction_with_sig_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnverifiedTransaction> {
        &mut self.transaction_with_sig
    }

    // bytes utx_hash = 2;

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

    // bytes public = 3;

    pub fn clear_public(&mut self) {
        self.public.clear();
    }

    // Param is passed by value, moved
    pub fn set_public(&mut self, v: ::std::vec::Vec<u8>) {
        self.public = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public
    }

    // Take field
    pub fn take_public(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public, ::std::vec::Vec::new())
    }

    pub fn get_public(&self) -> &[u8] {
        &self.public
    }

    fn get_public_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.public
    }

    fn mut_public_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public
    }
}

impl ::protobuf::Message for SignedTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction_with_sig {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction_with_sig)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.utx_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public)?;
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
        if let Some(ref v) = self.transaction_with_sig.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.utx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.utx_hash);
        }
        if !self.public.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.public);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction_with_sig.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.utx_hash.is_empty() {
            os.write_bytes(2, &self.utx_hash)?;
        }
        if !self.public.is_empty() {
            os.write_bytes(3, &self.public)?;
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

impl ::protobuf::MessageStatic for SignedTransaction {
    fn new() -> SignedTransaction {
        SignedTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnverifiedTransaction>>(
                    "transaction_with_sig",
                    SignedTransaction::get_transaction_with_sig_for_reflect,
                    SignedTransaction::mut_transaction_with_sig_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hash",
                    SignedTransaction::get_utx_hash_for_reflect,
                    SignedTransaction::mut_utx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public",
                    SignedTransaction::get_public_for_reflect,
                    SignedTransaction::mut_public_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedTransaction>(
                    "SignedTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedTransaction {
    fn clear(&mut self) {
        self.clear_transaction_with_sig();
        self.clear_utx_hash();
        self.clear_public();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Proof {
    // message fields
    pub votes: ::std::vec::Vec<u8>,
    pub field_type: ProofType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Proof {}

impl Proof {
    pub fn new() -> Proof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Proof {
        static mut instance: ::protobuf::lazy::Lazy<Proof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Proof,
        };
        unsafe {
            instance.get(Proof::new)
        }
    }

    // bytes votes = 1;

    pub fn clear_votes(&mut self) {
        self.votes.clear();
    }

    // Param is passed by value, moved
    pub fn set_votes(&mut self, v: ::std::vec::Vec<u8>) {
        self.votes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_votes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.votes
    }

    // Take field
    pub fn take_votes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.votes, ::std::vec::Vec::new())
    }

    pub fn get_votes(&self) -> &[u8] {
        &self.votes
    }

    fn get_votes_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.votes
    }

    fn mut_votes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.votes
    }

    // .ProofType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ProofType::Bft;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ProofType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> ProofType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &ProofType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ProofType {
        &mut self.field_type
    }
}

impl ::protobuf::Message for Proof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.votes)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
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
        if !self.votes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.votes);
        }
        if self.field_type != ProofType::Bft {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.votes.is_empty() {
            os.write_bytes(1, &self.votes)?;
        }
        if self.field_type != ProofType::Bft {
            os.write_enum(2, self.field_type.value())?;
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

impl ::protobuf::MessageStatic for Proof {
    fn new() -> Proof {
        Proof::new()
    }

    fn descriptor_static(_: ::std::option::Option<Proof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "votes",
                    Proof::get_votes_for_reflect,
                    Proof::mut_votes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProofType>>(
                    "type",
                    Proof::get_field_type_for_reflect,
                    Proof::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Proof>(
                    "Proof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Proof {
    fn clear(&mut self) {
        self.clear_votes();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Proof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Proof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    pub prevhash: ::std::vec::Vec<u8>,
    pub timestamp: u64,
    pub height: u64,
    pub transactions_root: ::std::vec::Vec<u8>,
    pub state_root: ::std::vec::Vec<u8>,
    pub receipts_root: ::std::vec::Vec<u8>,
    pub quota_used: u64,
    pub quota_limit: u64,
    pub proof: ::protobuf::SingularPtrField<Proof>,
    pub proposer: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockHeader {}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlockHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHeader,
        };
        unsafe {
            instance.get(BlockHeader::new)
        }
    }

    // bytes prevhash = 1;

    pub fn clear_prevhash(&mut self) {
        self.prevhash.clear();
    }

    // Param is passed by value, moved
    pub fn set_prevhash(&mut self, v: ::std::vec::Vec<u8>) {
        self.prevhash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prevhash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // Take field
    pub fn take_prevhash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.prevhash, ::std::vec::Vec::new())
    }

    pub fn get_prevhash(&self) -> &[u8] {
        &self.prevhash
    }

    fn get_prevhash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.prevhash
    }

    fn mut_prevhash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }

    // uint64 height = 3;

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

    // bytes transactions_root = 4;

    pub fn clear_transactions_root(&mut self) {
        self.transactions_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.transactions_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transactions_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // Take field
    pub fn take_transactions_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transactions_root, ::std::vec::Vec::new())
    }

    pub fn get_transactions_root(&self) -> &[u8] {
        &self.transactions_root
    }

    fn get_transactions_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transactions_root
    }

    fn mut_transactions_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // bytes state_root = 5;

    pub fn clear_state_root(&mut self) {
        self.state_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // Take field
    pub fn take_state_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.state_root, ::std::vec::Vec::new())
    }

    pub fn get_state_root(&self) -> &[u8] {
        &self.state_root
    }

    fn get_state_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.state_root
    }

    fn mut_state_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // bytes receipts_root = 6;

    pub fn clear_receipts_root(&mut self) {
        self.receipts_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipts_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.receipts_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipts_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // Take field
    pub fn take_receipts_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.receipts_root, ::std::vec::Vec::new())
    }

    pub fn get_receipts_root(&self) -> &[u8] {
        &self.receipts_root
    }

    fn get_receipts_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.receipts_root
    }

    fn mut_receipts_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // uint64 quota_used = 7;

    pub fn clear_quota_used(&mut self) {
        self.quota_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_quota_used(&mut self, v: u64) {
        self.quota_used = v;
    }

    pub fn get_quota_used(&self) -> u64 {
        self.quota_used
    }

    fn get_quota_used_for_reflect(&self) -> &u64 {
        &self.quota_used
    }

    fn mut_quota_used_for_reflect(&mut self) -> &mut u64 {
        &mut self.quota_used
    }

    // uint64 quota_limit = 8;

    pub fn clear_quota_limit(&mut self) {
        self.quota_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_quota_limit(&mut self, v: u64) {
        self.quota_limit = v;
    }

    pub fn get_quota_limit(&self) -> u64 {
        self.quota_limit
    }

    fn get_quota_limit_for_reflect(&self) -> &u64 {
        &self.quota_limit
    }

    fn mut_quota_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.quota_limit
    }

    // .Proof proof = 9;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: Proof) {
        self.proof = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut Proof {
        if self.proof.is_none() {
            self.proof.set_default();
        }
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> Proof {
        self.proof.take().unwrap_or_else(|| Proof::new())
    }

    pub fn get_proof(&self) -> &Proof {
        self.proof.as_ref().unwrap_or_else(|| Proof::default_instance())
    }

    fn get_proof_for_reflect(&self) -> &::protobuf::SingularPtrField<Proof> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Proof> {
        &mut self.proof
    }

    // bytes proposer = 10;

    pub fn clear_proposer(&mut self) {
        self.proposer.clear();
    }

    // Param is passed by value, moved
    pub fn set_proposer(&mut self, v: ::std::vec::Vec<u8>) {
        self.proposer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proposer(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proposer
    }

    // Take field
    pub fn take_proposer(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.proposer, ::std::vec::Vec::new())
    }

    pub fn get_proposer(&self) -> &[u8] {
        &self.proposer
    }

    fn get_proposer_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.proposer
    }

    fn mut_proposer_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proposer
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        for v in &self.proof {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.prevhash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transactions_root)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.receipts_root)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota_used = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota_limit = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proof)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proposer)?;
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
        if !self.prevhash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.prevhash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.transactions_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.transactions_root);
        }
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.state_root);
        }
        if !self.receipts_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.receipts_root);
        }
        if self.quota_used != 0 {
            my_size += ::protobuf::rt::value_size(7, self.quota_used, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.quota_limit != 0 {
            my_size += ::protobuf::rt::value_size(8, self.quota_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.proposer.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.proposer);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.prevhash.is_empty() {
            os.write_bytes(1, &self.prevhash)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        if self.height != 0 {
            os.write_uint64(3, self.height)?;
        }
        if !self.transactions_root.is_empty() {
            os.write_bytes(4, &self.transactions_root)?;
        }
        if !self.state_root.is_empty() {
            os.write_bytes(5, &self.state_root)?;
        }
        if !self.receipts_root.is_empty() {
            os.write_bytes(6, &self.receipts_root)?;
        }
        if self.quota_used != 0 {
            os.write_uint64(7, self.quota_used)?;
        }
        if self.quota_limit != 0 {
            os.write_uint64(8, self.quota_limit)?;
        }
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.proposer.is_empty() {
            os.write_bytes(10, &self.proposer)?;
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

impl ::protobuf::MessageStatic for BlockHeader {
    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prevhash",
                    BlockHeader::get_prevhash_for_reflect,
                    BlockHeader::mut_prevhash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    BlockHeader::get_timestamp_for_reflect,
                    BlockHeader::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockHeader::get_height_for_reflect,
                    BlockHeader::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transactions_root",
                    BlockHeader::get_transactions_root_for_reflect,
                    BlockHeader::mut_transactions_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    BlockHeader::get_state_root_for_reflect,
                    BlockHeader::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "receipts_root",
                    BlockHeader::get_receipts_root_for_reflect,
                    BlockHeader::mut_receipts_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota_used",
                    BlockHeader::get_quota_used_for_reflect,
                    BlockHeader::mut_quota_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota_limit",
                    BlockHeader::get_quota_limit_for_reflect,
                    BlockHeader::mut_quota_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Proof>>(
                    "proof",
                    BlockHeader::get_proof_for_reflect,
                    BlockHeader::mut_proof_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proposer",
                    BlockHeader::get_proposer_for_reflect,
                    BlockHeader::mut_proposer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHeader>(
                    "BlockHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.clear_prevhash();
        self.clear_timestamp();
        self.clear_height();
        self.clear_transactions_root();
        self.clear_state_root();
        self.clear_receipts_root();
        self.clear_quota_used();
        self.clear_quota_limit();
        self.clear_proof();
        self.clear_proposer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockBody {
    // message fields
    pub utx_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockBody {}

impl BlockBody {
    pub fn new() -> BlockBody {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockBody {
        static mut instance: ::protobuf::lazy::Lazy<BlockBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockBody,
        };
        unsafe {
            instance.get(BlockBody::new)
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

impl ::protobuf::Message for BlockBody {
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

impl ::protobuf::MessageStatic for BlockBody {
    fn new() -> BlockBody {
        BlockBody::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockBody>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utx_hashes",
                    BlockBody::get_utx_hashes_for_reflect,
                    BlockBody::mut_utx_hashes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockBody>(
                    "BlockBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockBody {
    fn clear(&mut self) {
        self.clear_utx_hashes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub version: u32,
    pub header: ::protobuf::SingularPtrField<BlockHeader>,
    pub body: ::protobuf::SingularPtrField<BlockBody>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }

    // uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }

    // .BlockHeader header = 2;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BlockHeader {
        self.header.take().unwrap_or_else(|| BlockHeader::new())
    }

    pub fn get_header(&self) -> &BlockHeader {
        self.header.as_ref().unwrap_or_else(|| BlockHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockHeader> {
        &mut self.header
    }

    // .BlockBody body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: BlockBody) {
        self.body = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut BlockBody {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> BlockBody {
        self.body.take().unwrap_or_else(|| BlockBody::new())
    }

    pub fn get_body(&self) -> &BlockBody {
        self.body.as_ref().unwrap_or_else(|| BlockBody::default_instance())
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockBody> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockBody> {
        &mut self.body
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.body {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body)?;
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
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.body.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Block::get_version_for_reflect,
                    Block::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockHeader>>(
                    "header",
                    Block::get_header_for_reflect,
                    Block::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockBody>>(
                    "body",
                    Block::get_body_for_reflect,
                    Block::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_header();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogEntry {
    // message fields
    pub address: ::std::vec::Vec<u8>,
    pub topics: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogEntry {}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry,
        };
        unsafe {
            instance.get(LogEntry::new)
        }
    }

    // bytes address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // repeated bytes topics = 2;

    pub fn clear_topics(&mut self) {
        self.topics.clear();
    }

    // Param is passed by value, moved
    pub fn set_topics(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.topics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_topics(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.topics
    }

    // Take field
    pub fn take_topics(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.topics, ::protobuf::RepeatedField::new())
    }

    pub fn get_topics(&self) -> &[::std::vec::Vec<u8>] {
        &self.topics
    }

    fn get_topics_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.topics
    }

    fn mut_topics_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.topics
    }

    // bytes data = 3;

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

impl ::protobuf::Message for LogEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.topics)?;
                },
                3 => {
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.address);
        }
        for value in &self.topics {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_bytes(1, &self.address)?;
        }
        for v in &self.topics {
            os.write_bytes(2, &v)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

impl ::protobuf::MessageStatic for LogEntry {
    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    LogEntry::get_address_for_reflect,
                    LogEntry::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "topics",
                    LogEntry::get_topics_for_reflect,
                    LogEntry::mut_topics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    LogEntry::get_data_for_reflect,
                    LogEntry::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry>(
                    "LogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_topics();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Receipt {
    // message fields
    pub state_root: ::std::vec::Vec<u8>,
    pub quota_used: ::std::vec::Vec<u8>,
    pub log_bloom: ::std::vec::Vec<u8>,
    pub logs: ::protobuf::RepeatedField<LogEntry>,
    pub error: ReceiptError,
    pub account_nonce: u64,
    pub transaction_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Receipt {}

impl Receipt {
    pub fn new() -> Receipt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Receipt {
        static mut instance: ::protobuf::lazy::Lazy<Receipt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Receipt,
        };
        unsafe {
            instance.get(Receipt::new)
        }
    }

    // bytes state_root = 1;

    pub fn clear_state_root(&mut self) {
        self.state_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // Take field
    pub fn take_state_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.state_root, ::std::vec::Vec::new())
    }

    pub fn get_state_root(&self) -> &[u8] {
        &self.state_root
    }

    fn get_state_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.state_root
    }

    fn mut_state_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // bytes quota_used = 2;

    pub fn clear_quota_used(&mut self) {
        self.quota_used.clear();
    }

    // Param is passed by value, moved
    pub fn set_quota_used(&mut self, v: ::std::vec::Vec<u8>) {
        self.quota_used = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_quota_used(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.quota_used
    }

    // Take field
    pub fn take_quota_used(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.quota_used, ::std::vec::Vec::new())
    }

    pub fn get_quota_used(&self) -> &[u8] {
        &self.quota_used
    }

    fn get_quota_used_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.quota_used
    }

    fn mut_quota_used_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.quota_used
    }

    // bytes log_bloom = 3;

    pub fn clear_log_bloom(&mut self) {
        self.log_bloom.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_bloom(&mut self, v: ::std::vec::Vec<u8>) {
        self.log_bloom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_bloom(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // Take field
    pub fn take_log_bloom(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.log_bloom, ::std::vec::Vec::new())
    }

    pub fn get_log_bloom(&self) -> &[u8] {
        &self.log_bloom
    }

    fn get_log_bloom_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.log_bloom
    }

    fn mut_log_bloom_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // repeated .LogEntry logs = 4;

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::protobuf::RepeatedField<LogEntry>) {
        self.logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.logs
    }

    // Take field
    pub fn take_logs(&mut self) -> ::protobuf::RepeatedField<LogEntry> {
        ::std::mem::replace(&mut self.logs, ::protobuf::RepeatedField::new())
    }

    pub fn get_logs(&self) -> &[LogEntry] {
        &self.logs
    }

    fn get_logs_for_reflect(&self) -> &::protobuf::RepeatedField<LogEntry> {
        &self.logs
    }

    fn mut_logs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.logs
    }

    // .ReceiptError error = 5;

    pub fn clear_error(&mut self) {
        self.error = ReceiptError::NotEnoughBaseQuota;
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ReceiptError) {
        self.error = v;
    }

    pub fn get_error(&self) -> ReceiptError {
        self.error
    }

    fn get_error_for_reflect(&self) -> &ReceiptError {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ReceiptError {
        &mut self.error
    }

    // uint64 account_nonce = 6;

    pub fn clear_account_nonce(&mut self) {
        self.account_nonce = 0;
    }

    // Param is passed by value, moved
    pub fn set_account_nonce(&mut self, v: u64) {
        self.account_nonce = v;
    }

    pub fn get_account_nonce(&self) -> u64 {
        self.account_nonce
    }

    fn get_account_nonce_for_reflect(&self) -> &u64 {
        &self.account_nonce
    }

    fn mut_account_nonce_for_reflect(&mut self) -> &mut u64 {
        &mut self.account_nonce
    }

    // bytes transaction_hash = 7;

    pub fn clear_transaction_hash(&mut self) {
        self.transaction_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.transaction_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_hash
    }

    // Take field
    pub fn take_transaction_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transaction_hash, ::std::vec::Vec::new())
    }

    pub fn get_transaction_hash(&self) -> &[u8] {
        &self.transaction_hash
    }

    fn get_transaction_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transaction_hash
    }

    fn mut_transaction_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_hash
    }
}

impl ::protobuf::Message for Receipt {
    fn is_initialized(&self) -> bool {
        for v in &self.logs {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.quota_used)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.log_bloom)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.logs)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.error = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_nonce = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transaction_hash)?;
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
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.state_root);
        }
        if !self.quota_used.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.quota_used);
        }
        if !self.log_bloom.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.log_bloom);
        }
        for value in &self.logs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.error != ReceiptError::NotEnoughBaseQuota {
            my_size += ::protobuf::rt::enum_size(5, self.error);
        }
        if self.account_nonce != 0 {
            my_size += ::protobuf::rt::value_size(6, self.account_nonce, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.transaction_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.transaction_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.state_root.is_empty() {
            os.write_bytes(1, &self.state_root)?;
        }
        if !self.quota_used.is_empty() {
            os.write_bytes(2, &self.quota_used)?;
        }
        if !self.log_bloom.is_empty() {
            os.write_bytes(3, &self.log_bloom)?;
        }
        for v in &self.logs {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.error != ReceiptError::NotEnoughBaseQuota {
            os.write_enum(5, self.error.value())?;
        }
        if self.account_nonce != 0 {
            os.write_uint64(6, self.account_nonce)?;
        }
        if !self.transaction_hash.is_empty() {
            os.write_bytes(7, &self.transaction_hash)?;
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

impl ::protobuf::MessageStatic for Receipt {
    fn new() -> Receipt {
        Receipt::new()
    }

    fn descriptor_static(_: ::std::option::Option<Receipt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    Receipt::get_state_root_for_reflect,
                    Receipt::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "quota_used",
                    Receipt::get_quota_used_for_reflect,
                    Receipt::mut_quota_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "log_bloom",
                    Receipt::get_log_bloom_for_reflect,
                    Receipt::mut_log_bloom_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LogEntry>>(
                    "logs",
                    Receipt::get_logs_for_reflect,
                    Receipt::mut_logs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReceiptError>>(
                    "error",
                    Receipt::get_error_for_reflect,
                    Receipt::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_nonce",
                    Receipt::get_account_nonce_for_reflect,
                    Receipt::mut_account_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transaction_hash",
                    Receipt::get_transaction_hash_for_reflect,
                    Receipt::mut_transaction_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Receipt>(
                    "Receipt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Receipt {
    fn clear(&mut self) {
        self.clear_state_root();
        self.clear_quota_used();
        self.clear_log_bloom();
        self.clear_logs();
        self.clear_error();
        self.clear_account_nonce();
        self.clear_transaction_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Receipt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Receipt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Crypto {
    SECP = 0,
}

impl ::protobuf::ProtobufEnum for Crypto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Crypto> {
        match value {
            0 => ::std::option::Option::Some(Crypto::SECP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Crypto] = &[
            Crypto::SECP,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Crypto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Crypto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Crypto {
}

impl ::std::default::Default for Crypto {
    fn default() -> Self {
        Crypto::SECP
    }
}

impl ::protobuf::reflect::ProtobufValue for Crypto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProofType {
    Bft = 0,
}

impl ::protobuf::ProtobufEnum for ProofType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProofType> {
        match value {
            0 => ::std::option::Option::Some(ProofType::Bft),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProofType] = &[
            ProofType::Bft,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ProofType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProofType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProofType {
}

impl ::std::default::Default for ProofType {
    fn default() -> Self {
        ProofType::Bft
    }
}

impl ::protobuf::reflect::ProtobufValue for ProofType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReceiptError {
    NotEnoughBaseQuota = 0,
    BlockQuotaLimitReached = 1,
    AccountQuotaLimitReached = 2,
    InvalidTransactionNonce = 3,
    NotEnoughCash = 4,
    NoTransactionPermission = 5,
    NoContractPermission = 6,
    NoCallPermission = 7,
    ExecutionInternal = 8,
    TransactionMalformed = 9,
    OutOfQuota = 10,
    BadJumpDestination = 11,
    BadInstruction = 12,
    StackUnderflow = 13,
    OutOfStack = 14,
    Internal = 15,
    MutableCallInStaticContext = 16,
    OutOfBounds = 17,
    Reverted = 18,
}

impl ::protobuf::ProtobufEnum for ReceiptError {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReceiptError> {
        match value {
            0 => ::std::option::Option::Some(ReceiptError::NotEnoughBaseQuota),
            1 => ::std::option::Option::Some(ReceiptError::BlockQuotaLimitReached),
            2 => ::std::option::Option::Some(ReceiptError::AccountQuotaLimitReached),
            3 => ::std::option::Option::Some(ReceiptError::InvalidTransactionNonce),
            4 => ::std::option::Option::Some(ReceiptError::NotEnoughCash),
            5 => ::std::option::Option::Some(ReceiptError::NoTransactionPermission),
            6 => ::std::option::Option::Some(ReceiptError::NoContractPermission),
            7 => ::std::option::Option::Some(ReceiptError::NoCallPermission),
            8 => ::std::option::Option::Some(ReceiptError::ExecutionInternal),
            9 => ::std::option::Option::Some(ReceiptError::TransactionMalformed),
            10 => ::std::option::Option::Some(ReceiptError::OutOfQuota),
            11 => ::std::option::Option::Some(ReceiptError::BadJumpDestination),
            12 => ::std::option::Option::Some(ReceiptError::BadInstruction),
            13 => ::std::option::Option::Some(ReceiptError::StackUnderflow),
            14 => ::std::option::Option::Some(ReceiptError::OutOfStack),
            15 => ::std::option::Option::Some(ReceiptError::Internal),
            16 => ::std::option::Option::Some(ReceiptError::MutableCallInStaticContext),
            17 => ::std::option::Option::Some(ReceiptError::OutOfBounds),
            18 => ::std::option::Option::Some(ReceiptError::Reverted),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReceiptError] = &[
            ReceiptError::NotEnoughBaseQuota,
            ReceiptError::BlockQuotaLimitReached,
            ReceiptError::AccountQuotaLimitReached,
            ReceiptError::InvalidTransactionNonce,
            ReceiptError::NotEnoughCash,
            ReceiptError::NoTransactionPermission,
            ReceiptError::NoContractPermission,
            ReceiptError::NoCallPermission,
            ReceiptError::ExecutionInternal,
            ReceiptError::TransactionMalformed,
            ReceiptError::OutOfQuota,
            ReceiptError::BadJumpDestination,
            ReceiptError::BadInstruction,
            ReceiptError::StackUnderflow,
            ReceiptError::OutOfStack,
            ReceiptError::Internal,
            ReceiptError::MutableCallInStaticContext,
            ReceiptError::OutOfBounds,
            ReceiptError::Reverted,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReceiptError>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReceiptError", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReceiptError {
}

impl ::std::default::Default for ReceiptError {
    fn default() -> Self {
        ReceiptError::NotEnoughBaseQuota
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10blockchain.proto\"\xd4\x01\n\x0bTransaction\x12\x0e\n\x02to\x18\
    \x01\x20\x01(\x0cR\x02to\x12\x14\n\x05nonce\x18\x02\x20\x01(\tR\x05nonce\
    \x12\x14\n\x05quota\x18\x03\x20\x01(\x04R\x05quota\x12*\n\x11valid_until\
    _block\x18\x04\x20\x01(\x04R\x0fvalidUntilBlock\x12\x12\n\x04data\x18\
    \x05\x20\x01(\x0cR\x04data\x12\x14\n\x05value\x18\x06\x20\x01(\x0cR\x05v\
    alue\x12\x19\n\x08chain_id\x18\x07\x20\x01(\x0cR\x07chainId\x12\x18\n\
    \x07version\x18\x08\x20\x01(\rR\x07version\"\x86\x01\n\x15UnverifiedTran\
    saction\x12.\n\x0btransaction\x18\x01\x20\x01(\x0b2\x0c.TransactionR\x0b\
    transaction\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsignature\x12\
    \x1f\n\x06crypto\x18\x03\x20\x01(\x0e2\x07.CryptoR\x06crypto\"\x90\x01\n\
    \x11SignedTransaction\x12H\n\x14transaction_with_sig\x18\x01\x20\x01(\
    \x0b2\x16.UnverifiedTransactionR\x12transactionWithSig\x12\x19\n\x08utx_\
    hash\x18\x02\x20\x01(\x0cR\x07utxHash\x12\x16\n\x06public\x18\x03\x20\
    \x01(\x0cR\x06public\"=\n\x05Proof\x12\x14\n\x05votes\x18\x01\x20\x01(\
    \x0cR\x05votes\x12\x1e\n\x04type\x18\x02\x20\x01(\x0e2\n.ProofTypeR\x04t\
    ype\"\xca\x02\n\x0bBlockHeader\x12\x1a\n\x08prevhash\x18\x01\x20\x01(\
    \x0cR\x08prevhash\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimestamp\
    \x12\x16\n\x06height\x18\x03\x20\x01(\x04R\x06height\x12+\n\x11transacti\
    ons_root\x18\x04\x20\x01(\x0cR\x10transactionsRoot\x12\x1d\n\nstate_root\
    \x18\x05\x20\x01(\x0cR\tstateRoot\x12#\n\rreceipts_root\x18\x06\x20\x01(\
    \x0cR\x0creceiptsRoot\x12\x1d\n\nquota_used\x18\x07\x20\x01(\x04R\tquota\
    Used\x12\x1f\n\x0bquota_limit\x18\x08\x20\x01(\x04R\nquotaLimit\x12\x1c\
    \n\x05proof\x18\t\x20\x01(\x0b2\x06.ProofR\x05proof\x12\x1a\n\x08propose\
    r\x18\n\x20\x01(\x0cR\x08proposer\"*\n\tBlockBody\x12\x1d\n\nutx_hashes\
    \x18\x01\x20\x03(\x0cR\tutxHashes\"g\n\x05Block\x12\x18\n\x07version\x18\
    \x01\x20\x01(\rR\x07version\x12$\n\x06header\x18\x02\x20\x01(\x0b2\x0c.B\
    lockHeaderR\x06header\x12\x1e\n\x04body\x18\x03\x20\x01(\x0b2\n.BlockBod\
    yR\x04body\"P\n\x08LogEntry\x12\x18\n\x07address\x18\x01\x20\x01(\x0cR\
    \x07address\x12\x16\n\x06topics\x18\x02\x20\x03(\x0cR\x06topics\x12\x12\
    \n\x04data\x18\x03\x20\x01(\x0cR\x04data\"\xf8\x01\n\x07Receipt\x12\x1d\
    \n\nstate_root\x18\x01\x20\x01(\x0cR\tstateRoot\x12\x1d\n\nquota_used\
    \x18\x02\x20\x01(\x0cR\tquotaUsed\x12\x1b\n\tlog_bloom\x18\x03\x20\x01(\
    \x0cR\x08logBloom\x12\x1d\n\x04logs\x18\x04\x20\x03(\x0b2\t.LogEntryR\
    \x04logs\x12#\n\x05error\x18\x05\x20\x01(\x0e2\r.ReceiptErrorR\x05error\
    \x12#\n\raccount_nonce\x18\x06\x20\x01(\x04R\x0caccountNonce\x12)\n\x10t\
    ransaction_hash\x18\x07\x20\x01(\x0cR\x0ftransactionHash*\x12\n\x06Crypt\
    o\x12\x08\n\x04SECP\x10\0*\x14\n\tProofType\x12\x07\n\x03Bft\x10\0*\xbb\
    \x03\n\x0cReceiptError\x12\x16\n\x12NotEnoughBaseQuota\x10\0\x12\x1a\n\
    \x16BlockQuotaLimitReached\x10\x01\x12\x1c\n\x18AccountQuotaLimitReached\
    \x10\x02\x12\x1b\n\x17InvalidTransactionNonce\x10\x03\x12\x11\n\rNotEnou\
    ghCash\x10\x04\x12\x1b\n\x17NoTransactionPermission\x10\x05\x12\x18\n\
    \x14NoContractPermission\x10\x06\x12\x14\n\x10NoCallPermission\x10\x07\
    \x12\x15\n\x11ExecutionInternal\x10\x08\x12\x18\n\x14TransactionMalforme\
    d\x10\t\x12\x0e\n\nOutOfQuota\x10\n\x12\x16\n\x12BadJumpDestination\x10\
    \x0b\x12\x12\n\x0eBadInstruction\x10\x0c\x12\x12\n\x0eStackUnderflow\x10\
    \r\x12\x0e\n\nOutOfStack\x10\x0e\x12\x0c\n\x08Internal\x10\x0f\x12\x1e\n\
    \x1aMutableCallInStaticContext\x10\x10\x12\x0f\n\x0bOutOfBounds\x10\x11\
    \x12\x0c\n\x08Reverted\x10\x12J\x87%\n\x06\x12\x04\0\0\x7f\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n1\n\x02\x04\0\x12\x04\x03\0\x10\x01\x1a%\x20tr\
    ansaction\x20-\x20232\x20+\x20len(data)\x20bytes\n\n\n\n\x03\x04\0\x01\
    \x12\x03\x03\x08\x13\n;\n\x04\x04\0\x02\0\x12\x03\x05\x04\x11\x1a.\x20by\
    tes20(Address)\x20-\x20address\x20deal\x20with\x20this\x20tx\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\x05\x04\x03\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\n\x0c\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x05\x0f\x10\n\x1c\n\x04\x04\0\x02\x01\x12\x03\
    \x07\x04\x15\x1a\x0f\x20length\x20<=\x20128\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x07\x04\x05\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\
    \x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x0b\x10\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x07\x13\x14\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\
    \x04\x15\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x08\x04\x07\x15\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x08\x0b\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08\x13\x14\n\x0b\
    \n\x04\x04\0\x02\x03\x12\x03\t\x04!\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\
    \t\x04\x08\x15\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\t\x04\n\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\t\x0b\x1c\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\t\x1f\x20\n\x0b\n\x04\x04\0\x02\x04\x12\x03\n\x04\x13\n\r\n\x05\x04\
    \0\x02\x04\x04\x12\x04\n\x04\t!\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\n\
    \x04\t\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\n\n\x0e\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\n\x11\x12\n\x1f\n\x04\x04\0\x02\x05\x12\x03\x0c\x04\
    \x14\x1a\x12\x20bytes32(uint256)\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\
    \x0c\x04\n\x13\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x0c\x04\t\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x03\x0c\n\x0f\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03\x0c\x12\x13\n\x1f\n\x04\x04\0\x02\x06\x12\x03\x0e\x04\x17\x1a\
    \x12\x20bytes32(uint256)\n\n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x0e\x04\
    \x0c\x14\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x0e\x04\t\n\x0c\n\x05\x04\
    \0\x02\x06\x01\x12\x03\x0e\n\x12\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\
    \x0e\x15\x16\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x0f\x04\x17\n\r\n\x05\x04\
    \0\x02\x07\x04\x12\x04\x0f\x04\x0e\x17\n\x0c\n\x05\x04\0\x02\x07\x05\x12\
    \x03\x0f\x04\n\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0f\x0b\x12\n\x0c\n\
    \x05\x04\0\x02\x07\x03\x12\x03\x0f\x15\x16\n\n\n\x02\x05\0\x12\x04\x12\0\
    \x14\x01\n\n\n\x03\x05\0\x01\x12\x03\x12\x05\x0b\n\x0b\n\x04\x05\0\x02\0\
    \x12\x03\x13\x04\r\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x13\x04\x08\n\x0c\
    \n\x05\x05\0\x02\0\x02\x12\x03\x13\x0b\x0c\n%\n\x02\x04\x01\x12\x04\x17\
    \0\x1c\x01\x1a\x19\x20user\x20use\x20this\x20structure\n\n\n\n\x03\x04\
    \x01\x01\x12\x03\x17\x08\x1d\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x18\x04\
    \x20\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x18\x04\x17\x1f\n\x0c\n\x05\x04\
    \x01\x02\0\x06\x12\x03\x18\x04\x0f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x18\x10\x1b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x18\x1e\x1f\n'\n\x04\
    \x04\x01\x02\x01\x12\x03\x1a\x04\x18\x1a\x1a\x20sign\x20hash\x20of\x20Tr\
    ansaction\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1a\x04\x18\x20\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03\x1a\x04\t\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x1a\n\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1a\x16\
    \x17\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1b\x04\x16\n\r\n\x05\x04\x01\
    \x02\x02\x04\x12\x04\x1b\x04\x1a\x18\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\
    \x03\x1b\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1b\x0b\x11\n\x0c\
    \n\x05\x04\x01\x02\x02\x03\x12\x03\x1b\x14\x15\n\"\n\x02\x04\x02\x12\x04\
    \x1f\0%\x01\x1a\x16\x20transaction\x20in\x20chain\n\n\n\n\x03\x04\x02\
    \x01\x12\x03\x1f\x08\x19\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x20\x043\n\r\
    \n\x05\x04\x02\x02\0\x04\x12\x04\x20\x04\x1f\x1b\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03\x20\x04\x19\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x20\x1a\
    .\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x2012\n9\n\x04\x04\x02\x02\x01\
    \x12\x03\"\x04\x17\x1a,\x20bytes32(H256)\x20-\x20UnverifiedTransaction\
    \x20hash\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\"\x04\x203\n\x0c\n\x05\
    \x04\x02\x02\x01\x05\x12\x03\"\x04\t\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03\"\n\x12\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\"\x15\x16\n&\n\x04\
    \x04\x02\x02\x02\x12\x03$\x04\x15\x1a\x19\x20public\x20key\x20of\x20sign\
    ature\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04$\x04\"\x17\n\x0c\n\x05\x04\
    \x02\x02\x02\x05\x12\x03$\x04\t\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03$\
    \n\x10\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03$\x13\x14\n\x13\n\x02\x05\
    \x01\x12\x04(\0*\x01\x1a\x07\x20block\n\n\n\n\x03\x05\x01\x01\x12\x03(\
    \x05\x0e\n\x0b\n\x04\x05\x01\x02\0\x12\x03)\x04\x0c\n\x0c\n\x05\x05\x01\
    \x02\0\x01\x12\x03)\x04\x07\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03)\n\x0b\
    \n\n\n\x02\x04\x03\x12\x04,\00\x01\n\n\n\x03\x04\x03\x01\x12\x03,\x08\r\
    \n$\n\x04\x04\x03\x02\0\x12\x03.\x04\x14\x1a\x17\x20produced\x20by\x20Co\
    nsensus\n\n\r\n\x05\x04\x03\x02\0\x04\x12\x04.\x04,\x0f\n\x0c\n\x05\x04\
    \x03\x02\0\x05\x12\x03.\x04\t\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03.\n\
    \x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03.\x12\x13\n\x0b\n\x04\x04\x03\
    \x02\x01\x12\x03/\x04\x17\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04/\x04.\
    \x14\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03/\x04\r\n\x0c\n\x05\x04\x03\
    \x02\x01\x01\x12\x03/\x0e\x12\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03/\
    \x15\x16\n\x1e\n\x02\x04\x04\x12\x043\0D\x01\x1a\x12\x20180\x20+\x20len(\
    proof)\n\n\n\n\x03\x04\x04\x01\x12\x033\x08\x13\n8\n\x04\x04\x04\x02\0\
    \x12\x035\x04\x17\x1a+\x20bytes32(H256)\x20-\x20hash\x20of\x20prev\x20bl\
    ock\x20header\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x045\x043\x15\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x035\x04\t\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x035\n\x12\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x035\x15\x16\n\x20\n\x04\
    \x04\x04\x02\x01\x12\x037\x04\x19\x1a\x13\x20ms\x20since\x201970-1-1\n\n\
    \r\n\x05\x04\x04\x02\x01\x04\x12\x047\x045\x17\n\x0c\n\x05\x04\x04\x02\
    \x01\x05\x12\x037\x04\n\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x037\x0b\x14\
    \n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x037\x17\x18\n\x0b\n\x04\x04\x04\
    \x02\x02\x12\x038\x04\x16\n\r\n\x05\x04\x04\x02\x02\x04\x12\x048\x047\
    \x19\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x038\x04\n\n\x0c\n\x05\x04\x04\
    \x02\x02\x01\x12\x038\x0b\x11\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x038\
    \x14\x15\n\x1c\n\x04\x04\x04\x02\x03\x12\x03:\x04\x20\x1a\x0f\x20bytes32\
    (H256)\n\n\r\n\x05\x04\x04\x02\x03\x04\x12\x04:\x048\x16\n\x0c\n\x05\x04\
    \x04\x02\x03\x05\x12\x03:\x04\t\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03:\
    \n\x1b\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03:\x1e\x1f\n\x1c\n\x04\x04\
    \x04\x02\x04\x12\x03<\x04\x19\x1a\x0f\x20bytes32(H256)\n\n\r\n\x05\x04\
    \x04\x02\x04\x04\x12\x04<\x04:\x20\n\x0c\n\x05\x04\x04\x02\x04\x05\x12\
    \x03<\x04\t\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03<\n\x14\n\x0c\n\x05\
    \x04\x04\x02\x04\x03\x12\x03<\x17\x18\n\x1c\n\x04\x04\x04\x02\x05\x12\
    \x03>\x04\x1c\x1a\x0f\x20bytes32(H256)\n\n\r\n\x05\x04\x04\x02\x05\x04\
    \x12\x04>\x04<\x19\n\x0c\n\x05\x04\x04\x02\x05\x05\x12\x03>\x04\t\n\x0c\
    \n\x05\x04\x04\x02\x05\x01\x12\x03>\n\x17\n\x0c\n\x05\x04\x04\x02\x05\
    \x03\x12\x03>\x1a\x1b\n\x0b\n\x04\x04\x04\x02\x06\x12\x03?\x04\x1a\n\r\n\
    \x05\x04\x04\x02\x06\x04\x12\x04?\x04>\x1c\n\x0c\n\x05\x04\x04\x02\x06\
    \x05\x12\x03?\x04\n\n\x0c\n\x05\x04\x04\x02\x06\x01\x12\x03?\x0b\x15\n\
    \x0c\n\x05\x04\x04\x02\x06\x03\x12\x03?\x18\x19\n\x0b\n\x04\x04\x04\x02\
    \x07\x12\x03@\x04\x1b\n\r\n\x05\x04\x04\x02\x07\x04\x12\x04@\x04?\x1a\n\
    \x0c\n\x05\x04\x04\x02\x07\x05\x12\x03@\x04\n\n\x0c\n\x05\x04\x04\x02\
    \x07\x01\x12\x03@\x0b\x16\n\x0c\n\x05\x04\x04\x02\x07\x03\x12\x03@\x19\
    \x1a\n\x0b\n\x04\x04\x04\x02\x08\x12\x03A\x04\x14\n\r\n\x05\x04\x04\x02\
    \x08\x04\x12\x04A\x04@\x1b\n\x0c\n\x05\x04\x04\x02\x08\x06\x12\x03A\x04\
    \t\n\x0c\n\x05\x04\x04\x02\x08\x01\x12\x03A\n\x0f\n\x0c\n\x05\x04\x04\
    \x02\x08\x03\x12\x03A\x12\x13\n,\n\x04\x04\x04\x02\t\x12\x03C\x04\x18\
    \x1a\x1f\x20bytes20\x20-\x20address\x20of\x20proposer\n\n\r\n\x05\x04\
    \x04\x02\t\x04\x12\x04C\x04A\x14\n\x0c\n\x05\x04\x04\x02\t\x05\x12\x03C\
    \x04\t\n\x0c\n\x05\x04\x04\x02\t\x01\x12\x03C\n\x12\n\x0c\n\x05\x04\x04\
    \x02\t\x03\x12\x03C\x15\x17\n\n\n\x02\x04\x05\x12\x04F\0I\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03F\x08\x11\n0\n\x04\x04\x05\x02\0\x12\x03H\x04\"\x1a#\
    \x20bytes32(H256)\x20-\x20array\x20of\x20utx\x20hash\n\n\x0c\n\x05\x04\
    \x05\x02\0\x04\x12\x03H\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03H\r\
    \x12\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03H\x13\x1d\n\x0c\n\x05\x04\x05\
    \x02\0\x03\x12\x03H\x20!\n*\n\x02\x04\x06\x12\x04L\0P\x01\x1a\x1e\x20pro\
    posal\x20block\x20same\x20as\x20Block\n\n\n\n\x03\x04\x06\x01\x12\x03L\
    \x08\r\n\x0b\n\x04\x04\x06\x02\0\x12\x03M\x04\x17\n\r\n\x05\x04\x06\x02\
    \0\x04\x12\x04M\x04L\x0f\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03M\x04\n\n\
    \x0c\n\x05\x04\x06\x02\0\x01\x12\x03M\x0b\x12\n\x0c\n\x05\x04\x06\x02\0\
    \x03\x12\x03M\x15\x16\n\x0b\n\x04\x04\x06\x02\x01\x12\x03N\x04\x1b\n\r\n\
    \x05\x04\x06\x02\x01\x04\x12\x04N\x04M\x17\n\x0c\n\x05\x04\x06\x02\x01\
    \x06\x12\x03N\x04\x0f\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03N\x10\x16\n\
    \x0c\n\x05\x04\x06\x02\x01\x03\x12\x03N\x19\x1a\n\x0b\n\x04\x04\x06\x02\
    \x02\x12\x03O\x04\x17\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04O\x04N\x1b\n\
    \x0c\n\x05\x04\x06\x02\x02\x06\x12\x03O\x04\r\n\x0c\n\x05\x04\x06\x02\
    \x02\x01\x12\x03O\x0e\x12\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03O\x15\
    \x16\n\x16\n\x02\x05\x02\x12\x04S\0i\x01\x1a\n\x20executor\n\n\n\n\x03\
    \x05\x02\x01\x12\x03S\x05\x11\n\x1c\n\x04\x05\x02\x02\0\x12\x03U\x04\x1b\
    \x1a\x0fExecutionError\n\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03U\x04\x16\
    \n\x0c\n\x05\x05\x02\x02\0\x02\x12\x03U\x19\x1a\n\x0b\n\x04\x05\x02\x02\
    \x01\x12\x03V\x04\x1f\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03V\x04\x1a\n\
    \x0c\n\x05\x05\x02\x02\x01\x02\x12\x03V\x1d\x1e\n\x0b\n\x04\x05\x02\x02\
    \x02\x12\x03W\x04!\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03W\x04\x1c\n\
    \x0c\n\x05\x05\x02\x02\x02\x02\x12\x03W\x1f\x20\n\x0b\n\x04\x05\x02\x02\
    \x03\x12\x03X\x04\x20\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03X\x04\x1b\n\
    \x0c\n\x05\x05\x02\x02\x03\x02\x12\x03X\x1e\x1f\n\x0b\n\x04\x05\x02\x02\
    \x04\x12\x03Y\x04\x16\n\x0c\n\x05\x05\x02\x02\x04\x01\x12\x03Y\x04\x11\n\
    \x0c\n\x05\x05\x02\x02\x04\x02\x12\x03Y\x14\x15\n\x0b\n\x04\x05\x02\x02\
    \x05\x12\x03Z\x04\x20\n\x0c\n\x05\x05\x02\x02\x05\x01\x12\x03Z\x04\x1b\n\
    \x0c\n\x05\x05\x02\x02\x05\x02\x12\x03Z\x1e\x1f\n\x0b\n\x04\x05\x02\x02\
    \x06\x12\x03[\x04\x1d\n\x0c\n\x05\x05\x02\x02\x06\x01\x12\x03[\x04\x18\n\
    \x0c\n\x05\x05\x02\x02\x06\x02\x12\x03[\x1b\x1c\n\x0b\n\x04\x05\x02\x02\
    \x07\x12\x03\\\x04\x19\n\x0c\n\x05\x05\x02\x02\x07\x01\x12\x03\\\x04\x14\
    \n\x0c\n\x05\x05\x02\x02\x07\x02\x12\x03\\\x17\x18\n\x0b\n\x04\x05\x02\
    \x02\x08\x12\x03]\x04\x1a\n\x0c\n\x05\x05\x02\x02\x08\x01\x12\x03]\x04\
    \x15\n\x0c\n\x05\x05\x02\x02\x08\x02\x12\x03]\x18\x19\n\x0b\n\x04\x05\
    \x02\x02\t\x12\x03^\x04\x1d\n\x0c\n\x05\x05\x02\x02\t\x01\x12\x03^\x04\
    \x18\n\x0c\n\x05\x05\x02\x02\t\x02\x12\x03^\x1b\x1c\n\x16\n\x04\x05\x02\
    \x02\n\x12\x03`\x04\x14\x1a\tEvmError\n\n\x0c\n\x05\x05\x02\x02\n\x01\
    \x12\x03`\x04\x0e\n\x0c\n\x05\x05\x02\x02\n\x02\x12\x03`\x11\x13\n\x0b\n\
    \x04\x05\x02\x02\x0b\x12\x03a\x04\x1c\n\x0c\n\x05\x05\x02\x02\x0b\x01\
    \x12\x03a\x04\x16\n\x0c\n\x05\x05\x02\x02\x0b\x02\x12\x03a\x19\x1b\n\x0b\
    \n\x04\x05\x02\x02\x0c\x12\x03b\x04\x18\n\x0c\n\x05\x05\x02\x02\x0c\x01\
    \x12\x03b\x04\x12\n\x0c\n\x05\x05\x02\x02\x0c\x02\x12\x03b\x15\x17\n\x0b\
    \n\x04\x05\x02\x02\r\x12\x03c\x04\x18\n\x0c\n\x05\x05\x02\x02\r\x01\x12\
    \x03c\x04\x12\n\x0c\n\x05\x05\x02\x02\r\x02\x12\x03c\x15\x17\n\x0b\n\x04\
    \x05\x02\x02\x0e\x12\x03d\x04\x14\n\x0c\n\x05\x05\x02\x02\x0e\x01\x12\
    \x03d\x04\x0e\n\x0c\n\x05\x05\x02\x02\x0e\x02\x12\x03d\x11\x13\n\x0b\n\
    \x04\x05\x02\x02\x0f\x12\x03e\x04\x12\n\x0c\n\x05\x05\x02\x02\x0f\x01\
    \x12\x03e\x04\x0c\n\x0c\n\x05\x05\x02\x02\x0f\x02\x12\x03e\x0f\x11\n\x0b\
    \n\x04\x05\x02\x02\x10\x12\x03f\x04$\n\x0c\n\x05\x05\x02\x02\x10\x01\x12\
    \x03f\x04\x1e\n\x0c\n\x05\x05\x02\x02\x10\x02\x12\x03f!#\n\x0b\n\x04\x05\
    \x02\x02\x11\x12\x03g\x04\x15\n\x0c\n\x05\x05\x02\x02\x11\x01\x12\x03g\
    \x04\x0f\n\x0c\n\x05\x05\x02\x02\x11\x02\x12\x03g\x12\x14\n\x0b\n\x04\
    \x05\x02\x02\x12\x12\x03h\x04\x12\n\x0c\n\x05\x05\x02\x02\x12\x01\x12\
    \x03h\x04\x0c\n\x0c\n\x05\x05\x02\x02\x12\x02\x12\x03h\x0f\x11\n\n\n\x02\
    \x04\x07\x12\x04k\0q\x01\n\n\n\x03\x04\x07\x01\x12\x03k\x08\x10\nD\n\x04\
    \x04\x07\x02\0\x12\x03m\x04\x16\x1a7\x20bytes20(Address)\x20-\x20contrac\
    t\x20address\x20deal\x20with\x20this\x20tx\n\n\r\n\x05\x04\x07\x02\0\x04\
    \x12\x04m\x04k\x12\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03m\x04\t\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03m\n\x11\n\x0c\n\x05\x04\x07\x02\0\x03\x12\
    \x03m\x14\x15\n\x1c\n\x04\x04\x07\x02\x01\x12\x03o\x04\x1e\x1a\x0f\x20by\
    tes32(H256)\n\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03o\x04\x0c\n\x0c\n\
    \x05\x04\x07\x02\x01\x05\x12\x03o\r\x12\n\x0c\n\x05\x04\x07\x02\x01\x01\
    \x12\x03o\x13\x19\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03o\x1c\x1d\n\x0b\
    \n\x04\x04\x07\x02\x02\x12\x03p\x04\x13\n\r\n\x05\x04\x07\x02\x02\x04\
    \x12\x04p\x04o\x1e\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03p\x04\t\n\x0c\
    \n\x05\x04\x07\x02\x02\x01\x12\x03p\n\x0e\n\x0c\n\x05\x04\x07\x02\x02\
    \x03\x12\x03p\x11\x12\n\n\n\x02\x04\x08\x12\x04s\0\x7f\x01\n\n\n\x03\x04\
    \x08\x01\x12\x03s\x08\x0f\n\x1c\n\x04\x04\x08\x02\0\x12\x03u\x04\x19\x1a\
    \x0f\x20bytes32(H256)\n\n\r\n\x05\x04\x08\x02\0\x04\x12\x04u\x04s\x11\n\
    \x0c\n\x05\x04\x08\x02\0\x05\x12\x03u\x04\t\n\x0c\n\x05\x04\x08\x02\0\
    \x01\x12\x03u\n\x14\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03u\x17\x18\n4\n\
    \x04\x04\x08\x02\x01\x12\x03w\x04\x19\x1a'\x20bytes32(U256)\x20-\x20quot\
    a\x20used\x20by\x20this\x20tx\n\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04w\
    \x04u\x19\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03w\x04\t\n\x0c\n\x05\x04\
    \x08\x02\x01\x01\x12\x03w\n\x14\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03w\
    \x17\x18\n\x20\n\x04\x04\x08\x02\x02\x12\x03y\x04\x18\x1a\x13\x20bytes25\
    6(2048bit)\n\n\r\n\x05\x04\x08\x02\x02\x04\x12\x04y\x04w\x19\n\x0c\n\x05\
    \x04\x08\x02\x02\x05\x12\x03y\x04\t\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\
    \x03y\n\x13\n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x03y\x16\x17\n\x0b\n\x04\
    \x04\x08\x02\x03\x12\x03z\x04\x1f\n\x0c\n\x05\x04\x08\x02\x03\x04\x12\
    \x03z\x04\x0c\n\x0c\n\x05\x04\x08\x02\x03\x06\x12\x03z\r\x15\n\x0c\n\x05\
    \x04\x08\x02\x03\x01\x12\x03z\x16\x1a\n\x0c\n\x05\x04\x08\x02\x03\x03\
    \x12\x03z\x1d\x1e\n\x0b\n\x04\x04\x08\x02\x04\x12\x03{\x04\x1b\n\r\n\x05\
    \x04\x08\x02\x04\x04\x12\x04{\x04z\x1f\n\x0c\n\x05\x04\x08\x02\x04\x06\
    \x12\x03{\x04\x10\n\x0c\n\x05\x04\x08\x02\x04\x01\x12\x03{\x11\x16\n\x0c\
    \n\x05\x04\x08\x02\x04\x03\x12\x03{\x19\x1a\n\x0b\n\x04\x04\x08\x02\x05\
    \x12\x03|\x04\x1d\n\r\n\x05\x04\x08\x02\x05\x04\x12\x04|\x04{\x1b\n\x0c\
    \n\x05\x04\x08\x02\x05\x05\x12\x03|\x04\n\n\x0c\n\x05\x04\x08\x02\x05\
    \x01\x12\x03|\x0b\x18\n\x0c\n\x05\x04\x08\x02\x05\x03\x12\x03|\x1b\x1c\n\
    \x1c\n\x04\x04\x08\x02\x06\x12\x03~\x04\x1f\x1a\x0f\x20bytes32(H256)\n\n\
    \r\n\x05\x04\x08\x02\x06\x04\x12\x04~\x04|\x1d\n\x0c\n\x05\x04\x08\x02\
    \x06\x05\x12\x03~\x04\t\n\x0c\n\x05\x04\x08\x02\x06\x01\x12\x03~\n\x1a\n\
    \x0c\n\x05\x04\x08\x02\x06\x03\x12\x03~\x1d\x1eb\x06proto3\
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
