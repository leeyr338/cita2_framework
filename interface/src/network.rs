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
pub struct UnverifiedTransactionHashes {
    // message fields
    pub NodeID: u32,
    pub UnverifiedTransactionHashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
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

    // uint32 NodeID = 1;

    pub fn clear_NodeID(&mut self) {
        self.NodeID = 0;
    }

    // Param is passed by value, moved
    pub fn set_NodeID(&mut self, v: u32) {
        self.NodeID = v;
    }

    pub fn get_NodeID(&self) -> u32 {
        self.NodeID
    }

    fn get_NodeID_for_reflect(&self) -> &u32 {
        &self.NodeID
    }

    fn mut_NodeID_for_reflect(&mut self) -> &mut u32 {
        &mut self.NodeID
    }

    // repeated bytes UnverifiedTransactionHashes = 2;

    pub fn clear_UnverifiedTransactionHashes(&mut self) {
        self.UnverifiedTransactionHashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_UnverifiedTransactionHashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.UnverifiedTransactionHashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_UnverifiedTransactionHashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.UnverifiedTransactionHashes
    }

    // Take field
    pub fn take_UnverifiedTransactionHashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.UnverifiedTransactionHashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_UnverifiedTransactionHashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.UnverifiedTransactionHashes
    }

    fn get_UnverifiedTransactionHashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.UnverifiedTransactionHashes
    }

    fn mut_UnverifiedTransactionHashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.UnverifiedTransactionHashes
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.NodeID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.UnverifiedTransactionHashes)?;
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
        if self.NodeID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.NodeID, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.UnverifiedTransactionHashes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.NodeID != 0 {
            os.write_uint32(1, self.NodeID)?;
        }
        for v in &self.UnverifiedTransactionHashes {
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "NodeID",
                    UnverifiedTransactionHashes::get_NodeID_for_reflect,
                    UnverifiedTransactionHashes::mut_NodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "UnverifiedTransactionHashes",
                    UnverifiedTransactionHashes::get_UnverifiedTransactionHashes_for_reflect,
                    UnverifiedTransactionHashes::mut_UnverifiedTransactionHashes_for_reflect,
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
        self.clear_NodeID();
        self.clear_UnverifiedTransactionHashes();
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
pub struct SyncRequest {
    // message fields
    pub NodeID: u32,
    pub heights: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncRequest {}

impl SyncRequest {
    pub fn new() -> SyncRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncRequest {
        static mut instance: ::protobuf::lazy::Lazy<SyncRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncRequest,
        };
        unsafe {
            instance.get(SyncRequest::new)
        }
    }

    // uint32 NodeID = 1;

    pub fn clear_NodeID(&mut self) {
        self.NodeID = 0;
    }

    // Param is passed by value, moved
    pub fn set_NodeID(&mut self, v: u32) {
        self.NodeID = v;
    }

    pub fn get_NodeID(&self) -> u32 {
        self.NodeID
    }

    fn get_NodeID_for_reflect(&self) -> &u32 {
        &self.NodeID
    }

    fn mut_NodeID_for_reflect(&mut self) -> &mut u32 {
        &mut self.NodeID
    }

    // repeated uint64 heights = 2;

    pub fn clear_heights(&mut self) {
        self.heights.clear();
    }

    // Param is passed by value, moved
    pub fn set_heights(&mut self, v: ::std::vec::Vec<u64>) {
        self.heights = v;
    }

    // Mutable pointer to the field.
    pub fn mut_heights(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.heights
    }

    // Take field
    pub fn take_heights(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.heights, ::std::vec::Vec::new())
    }

    pub fn get_heights(&self) -> &[u64] {
        &self.heights
    }

    fn get_heights_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.heights
    }

    fn mut_heights_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.heights
    }
}

impl ::protobuf::Message for SyncRequest {
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
                    self.NodeID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.heights)?;
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
        if self.NodeID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.NodeID, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.heights {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.NodeID != 0 {
            os.write_uint32(1, self.NodeID)?;
        }
        for v in &self.heights {
            os.write_uint64(2, *v)?;
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

impl ::protobuf::MessageStatic for SyncRequest {
    fn new() -> SyncRequest {
        SyncRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "NodeID",
                    SyncRequest::get_NodeID_for_reflect,
                    SyncRequest::mut_NodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "heights",
                    SyncRequest::get_heights_for_reflect,
                    SyncRequest::mut_heights_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncRequest>(
                    "SyncRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncRequest {
    fn clear(&mut self) {
        self.clear_NodeID();
        self.clear_heights();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncResponse {
    // message fields
    pub NodeID: u32,
    pub blocks: ::protobuf::RepeatedField<super::blockchain::Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncResponse {}

impl SyncResponse {
    pub fn new() -> SyncResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncResponse {
        static mut instance: ::protobuf::lazy::Lazy<SyncResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncResponse,
        };
        unsafe {
            instance.get(SyncResponse::new)
        }
    }

    // uint32 NodeID = 1;

    pub fn clear_NodeID(&mut self) {
        self.NodeID = 0;
    }

    // Param is passed by value, moved
    pub fn set_NodeID(&mut self, v: u32) {
        self.NodeID = v;
    }

    pub fn get_NodeID(&self) -> u32 {
        self.NodeID
    }

    fn get_NodeID_for_reflect(&self) -> &u32 {
        &self.NodeID
    }

    fn mut_NodeID_for_reflect(&mut self) -> &mut u32 {
        &mut self.NodeID
    }

    // repeated .Block blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<super::blockchain::Block>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::Block> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<super::blockchain::Block> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[super::blockchain::Block] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<super::blockchain::Block> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::Block> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for SyncResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.blocks {
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
                    self.NodeID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        if self.NodeID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.NodeID, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.NodeID != 0 {
            os.write_uint32(1, self.NodeID)?;
        }
        for v in &self.blocks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SyncResponse {
    fn new() -> SyncResponse {
        SyncResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "NodeID",
                    SyncResponse::get_NodeID_for_reflect,
                    SyncResponse::mut_NodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Block>>(
                    "blocks",
                    SyncResponse::get_blocks_for_reflect,
                    SyncResponse::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncResponse>(
                    "SyncResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncResponse {
    fn clear(&mut self) {
        self.clear_NodeID();
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rnetwork.proto\x1a\x10blockchain.proto\"w\n\x1bUnverifiedTransactionH\
    ashes\x12\x16\n\x06NodeID\x18\x01\x20\x01(\rR\x06NodeID\x12@\n\x1bUnveri\
    fiedTransactionHashes\x18\x02\x20\x03(\x0cR\x1bUnverifiedTransactionHash\
    es\"&\n\x10ConsensusMessage\x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04d\
    ata\"#\n\tNewStatus\x12\x16\n\x06height\x18\x01\x20\x01(\x04R\x06height\
    \"?\n\x0bSyncRequest\x12\x16\n\x06NodeID\x18\x01\x20\x01(\rR\x06NodeID\
    \x12\x18\n\x07heights\x18\x02\x20\x03(\x04R\x07heights\"F\n\x0cSyncRespo\
    nse\x12\x16\n\x06NodeID\x18\x01\x20\x01(\rR\x06NodeID\x12\x1e\n\x06block\
    s\x18\x02\x20\x03(\x0b2\x06.BlockR\x06blocks2\xe7\x02\n\x0eNetworkServic\
    e\x12B\n\x1cForwordUnverifiedTransaction\x12\x16.UnverifiedTransaction\
    \x1a\n.RpcStatus\x12K\n\x1fSendUnverifiedTransactionHashes\x12\x1c.Unver\
    ifiedTransactionHashes\x1a\n.RpcStatus\x12:\n\x19BroadcastConsensusMessa\
    ge\x12\x11.ConsensusMessage\x1a\n.RpcStatus\x12,\n\x12BroadcastNewStatus\
    \x12\n.NewStatus\x1a\n.RpcStatus\x12+\n\x0fSendSyncRequest\x12\x0c.SyncR\
    equest\x1a\n.RpcStatus\x12-\n\x10SendSyncResponse\x12\r.SyncResponse\x1a\
    \n.RpcStatusJ\x9a\t\n\x06\x12\x04\x01\0+\x01\n\x08\n\x01\x0c\x12\x03\x01\
    \0\x12\n\t\n\x02\x03\0\x12\x03\x03\x07\x19\n\n\n\x02\x04\0\x12\x04\x05\0\
    \x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\t$\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x06\x04>\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x06\x04\x05&\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x06\x1b!\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06<=\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x07\x04>\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x07\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\r\x12\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\x07\x1b6\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07<\
    =\n\n\n\x02\x04\x01\x12\x04\n\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\
    \x08\x18\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x04\x13\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x0b\x04\n\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x0b\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\n\x0e\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x0b\x11\x12\n\n\n\x02\x04\x02\x12\x04\x0e\0\
    \x10\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x11\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x0f\x04\x16\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x0f\x04\
    \x0e\x13\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03\x0f\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\
    \x0f\x14\x15\nD\n\x02\x04\x03\x12\x04\x13\0\x16\x01\x1a8/\x20FIXME:\x20M\
    ore\x20SyncRequest\x20messages\x20for\x20SendSyncRequest.\n\n\n\n\x03\
    \x04\x03\x01\x12\x03\x13\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x14\
    \x04*\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x14\x04\x13\x15\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x14\x19\x1f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x14()\n\x0b\n\x04\
    \x04\x03\x02\x01\x12\x03\x15\x04*\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\
    \x03\x15\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x15\r\x13\n\x0c\
    \n\x05\x04\x03\x02\x01\x01\x12\x03\x15\x19\x20\n\x0c\n\x05\x04\x03\x02\
    \x01\x03\x12\x03\x15()\nB\n\x02\x04\x04\x12\x04\x19\0\x1c\x01\x1a6/\x20F\
    IXME:\x20More\x20messages\x20messages\x20for\x20SendSyncResponse.\n\n\n\
    \n\x03\x04\x04\x01\x12\x03\x19\x08\x14\n\x0b\n\x04\x04\x04\x02\0\x12\x03\
    \x1a\x04*\n\r\n\x05\x04\x04\x02\0\x04\x12\x04\x1a\x04\x19\x16\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x03\x1a\x04\n\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x03\x1a\x19\x1f\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1a()\n\x0b\n\
    \x04\x04\x04\x02\x01\x12\x03\x1b\x04*\n\x0c\n\x05\x04\x04\x02\x01\x04\
    \x12\x03\x1b\x04\x0c\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03\x1b\r\x12\n\
    \x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1b\x19\x1f\n\x0c\n\x05\x04\x04\
    \x02\x01\x03\x12\x03\x1b()\n\x0e\n\x02\x06\0\x12\x04\x1f\0+\x01\x1a\x02/\
    \n\n\n\n\x03\x06\0\x01\x12\x03\x1f\x08\x16\n\x0b\n\x04\x06\0\x02\0\x12\
    \x03\x20\x04Q\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x20\x08$\n\x0c\n\x05\
    \x06\0\x02\0\x02\x12\x03\x20&;\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x20FO\
    \n\x0b\n\x04\x06\0\x02\x01\x12\x03\"\x04Z\n\x0c\n\x05\x06\0\x02\x01\x01\
    \x12\x03\"\x08'\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\")D\n\x0c\n\x05\
    \x06\0\x02\x01\x03\x12\x03\"OX\n\x0b\n\x04\x06\0\x02\x02\x12\x03$\x04I\n\
    \x0c\n\x05\x06\0\x02\x02\x01\x12\x03$\x08!\n\x0c\n\x05\x06\0\x02\x02\x02\
    \x12\x03$#3\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03$>G\n\x0b\n\x04\x06\0\
    \x02\x03\x12\x03&\x04;\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03&\x08\x1a\n\
    \x0c\n\x05\x06\0\x02\x03\x02\x12\x03&\x1c%\n\x0c\n\x05\x06\0\x02\x03\x03\
    \x12\x03&09\n\x0b\n\x04\x06\0\x02\x04\x12\x03(\x049\n\x0c\n\x05\x06\0\
    \x02\x04\x01\x12\x03(\x08\x17\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03(\x18\
    #\n\x0c\n\x05\x06\0\x02\x04\x03\x12\x03(.7\n\x0b\n\x04\x06\0\x02\x05\x12\
    \x03*\x04;\n\x0c\n\x05\x06\0\x02\x05\x01\x12\x03*\x08\x18\n\x0c\n\x05\
    \x06\0\x02\x05\x02\x12\x03*\x19%\n\x0c\n\x05\x06\0\x02\x05\x03\x12\x03*0\
    9b\x06proto3\
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
