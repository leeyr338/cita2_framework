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
pub struct StatusReq {
    // message fields
    pub node_id: u32,
    pub height: u64,
    pub block_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusReq {}

impl StatusReq {
    pub fn new() -> StatusReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusReq {
        static mut instance: ::protobuf::lazy::Lazy<StatusReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusReq,
        };
        unsafe {
            instance.get(StatusReq::new)
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

    // uint64 height = 2;

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

    // bytes block_hash = 3;

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

impl ::protobuf::Message for StatusReq {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                3 => {
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.block_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_uint32(1, self.node_id)?;
        }
        if self.height != 0 {
            os.write_uint64(2, self.height)?;
        }
        if !self.block_hash.is_empty() {
            os.write_bytes(3, &self.block_hash)?;
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

impl ::protobuf::MessageStatic for StatusReq {
    fn new() -> StatusReq {
        StatusReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_id",
                    StatusReq::get_node_id_for_reflect,
                    StatusReq::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    StatusReq::get_height_for_reflect,
                    StatusReq::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hash",
                    StatusReq::get_block_hash_for_reflect,
                    StatusReq::mut_block_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusReq>(
                    "StatusReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusReq {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_height();
        self.clear_block_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatusReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncRequest {
    // message fields
    pub node_id: u32,
    pub start_height: u64,
    pub end_height: u64,
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

    // uint64 start_height = 2;

    pub fn clear_start_height(&mut self) {
        self.start_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_height(&mut self, v: u64) {
        self.start_height = v;
    }

    pub fn get_start_height(&self) -> u64 {
        self.start_height
    }

    fn get_start_height_for_reflect(&self) -> &u64 {
        &self.start_height
    }

    fn mut_start_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_height
    }

    // uint64 end_height = 3;

    pub fn clear_end_height(&mut self) {
        self.end_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_end_height(&mut self, v: u64) {
        self.end_height = v;
    }

    pub fn get_end_height(&self) -> u64 {
        self.end_height
    }

    fn get_end_height_for_reflect(&self) -> &u64 {
        &self.end_height
    }

    fn mut_end_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.end_height
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
                    self.node_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.end_height = tmp;
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
        if self.start_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.end_height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_uint32(1, self.node_id)?;
        }
        if self.start_height != 0 {
            os.write_uint64(2, self.start_height)?;
        }
        if self.end_height != 0 {
            os.write_uint64(3, self.end_height)?;
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
                    "node_id",
                    SyncRequest::get_node_id_for_reflect,
                    SyncRequest::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_height",
                    SyncRequest::get_start_height_for_reflect,
                    SyncRequest::mut_start_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "end_height",
                    SyncRequest::get_end_height_for_reflect,
                    SyncRequest::mut_end_height_for_reflect,
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
        self.clear_node_id();
        self.clear_start_height();
        self.clear_end_height();
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
    pub node_id: u32,
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
                    self.node_id = tmp;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
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
        if self.node_id != 0 {
            os.write_uint32(1, self.node_id)?;
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
                    "node_id",
                    SyncResponse::get_node_id_for_reflect,
                    SyncResponse::mut_node_id_for_reflect,
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
        self.clear_node_id();
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

#[derive(PartialEq,Clone,Default)]
pub struct PeerCount {
    // message fields
    pub peer_count: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerCount {}

impl PeerCount {
    pub fn new() -> PeerCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerCount {
        static mut instance: ::protobuf::lazy::Lazy<PeerCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerCount,
        };
        unsafe {
            instance.get(PeerCount::new)
        }
    }

    // uint32 peer_count = 1;

    pub fn clear_peer_count(&mut self) {
        self.peer_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_peer_count(&mut self, v: u32) {
        self.peer_count = v;
    }

    pub fn get_peer_count(&self) -> u32 {
        self.peer_count
    }

    fn get_peer_count_for_reflect(&self) -> &u32 {
        &self.peer_count
    }

    fn mut_peer_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.peer_count
    }
}

impl ::protobuf::Message for PeerCount {
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
                    self.peer_count = tmp;
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
        if self.peer_count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.peer_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.peer_count != 0 {
            os.write_uint32(1, self.peer_count)?;
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

impl ::protobuf::MessageStatic for PeerCount {
    fn new() -> PeerCount {
        PeerCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "peer_count",
                    PeerCount::get_peer_count_for_reflect,
                    PeerCount::mut_peer_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerCount>(
                    "PeerCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerCount {
    fn clear(&mut self) {
        self.clear_peer_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nsync.proto\x1a\x10blockchain.proto\"[\n\tStatusReq\x12\x17\n\x07node\
    _id\x18\x01\x20\x01(\rR\x06nodeId\x12\x16\n\x06height\x18\x02\x20\x01(\
    \x04R\x06height\x12\x1d\n\nblock_hash\x18\x03\x20\x01(\x0cR\tblockHash\"\
    h\n\x0bSyncRequest\x12\x17\n\x07node_id\x18\x01\x20\x01(\rR\x06nodeId\
    \x12!\n\x0cstart_height\x18\x02\x20\x01(\x04R\x0bstartHeight\x12\x1d\n\n\
    end_height\x18\x03\x20\x01(\x04R\tendHeight\"G\n\x0cSyncResponse\x12\x17\
    \n\x07node_id\x18\x01\x20\x01(\rR\x06nodeId\x12\x1e\n\x06blocks\x18\x02\
    \x20\x03(\x0b2\x06.BlockR\x06blocks\"*\n\tPeerCount\x12\x1d\n\npeer_coun\
    t\x18\x01\x20\x01(\rR\tpeerCount2\xb7\x01\n\x0bSyncService\x12&\n\x0cUpd\
    ateStatus\x12\n.StatusReq\x1a\n.RpcStatus\x12+\n\x0fProcSyncRequest\x12\
    \x0c.SyncRequest\x1a\n.RpcStatus\x12-\n\x10ProcSyncResponse\x12\r.SyncRe\
    sponse\x1a\n.RpcStatus\x12$\n\x0cGetPeerCount\x12\x08.RpcNone\x1a\n.Peer\
    CountJ\x9c\x08\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\t\n\x02\x03\0\x12\x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\n\x01\
    \n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x05\x04(\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x13\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x05\x13\x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05&'\nA\n\x04\x04\0\
    \x02\x01\x12\x03\x08\x04(\x1a4\x20FIXME:\x20new\x20message\x20for\x20Sta\
    tus\x20or\x20in\x20this\x20message\x20?\n\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04\x08\x04\x05(\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x04\n\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\x13\x19\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x08&'\n\x0b\n\x04\x04\0\x02\x02\x12\x03\t\x04(\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04\t\x04\x08(\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\t\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\t\x13\x1d\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\t&'\n\n\n\x02\x04\x01\x12\x04\x0c\0\x10\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\x08\x13\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\r\x04(\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\r\x04\x0c\x15\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\r\x13\x1a\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\r&'\n\x0b\n\x04\
    \x04\x01\x02\x01\x12\x03\x0e\x04(\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\
    \x0e\x04\r(\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0e\x04\n\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x0e\x13\x1f\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x0e&'\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0f\x04(\n\r\n\
    \x05\x04\x01\x02\x02\x04\x12\x04\x0f\x04\x0e(\n\x0c\n\x05\x04\x01\x02\
    \x02\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0f\
    \x13\x1d\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0f&'\n\n\n\x02\x04\x02\
    \x12\x04\x12\0\x15\x01\n\n\n\x03\x04\x02\x01\x12\x03\x12\x08\x14\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03\x13\x04(\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\
    \x13\x04\x12\x16\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x13\x04\n\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x13\x15\x1c\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x13&'\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x14\x04(\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x03\x14\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x06\
    \x12\x03\x14\r\x12\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x14\x15\x1b\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x14&'\n\n\n\x02\x04\x03\x12\x04\
    \x17\0\x19\x01\n\n\n\x03\x04\x03\x01\x12\x03\x17\x08\x11\n\x0b\n\x04\x04\
    \x03\x02\0\x12\x03\x18\x04(\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x18\x04\
    \x17\x13\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x18\x04\n\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03\x18\x15\x1f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\
    \x18&'\n\n\n\x02\x06\0\x12\x04\x1b\0\x20\x01\n\n\n\x03\x06\0\x01\x12\x03\
    \x1b\x08\x13\n\x0b\n\x04\x06\0\x02\0\x12\x03\x1c\x023\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x1c\x06\x12\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x1c\
    \x14\x1d\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x1c(1\n\x0b\n\x04\x06\0\x02\
    \x01\x12\x03\x1d\x028\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x1d\x06\x15\
    \n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x1d\x17\"\n\x0c\n\x05\x06\0\x02\
    \x01\x03\x12\x03\x1d-6\n\x0b\n\x04\x06\0\x02\x02\x12\x03\x1e\x02:\n\x0c\
    \n\x05\x06\0\x02\x02\x01\x12\x03\x1e\x06\x16\n\x0c\n\x05\x06\0\x02\x02\
    \x02\x12\x03\x1e\x18$\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x1e/8\n\x0b\
    \n\x04\x06\0\x02\x03\x12\x03\x1f\x021\n\x0c\n\x05\x06\0\x02\x03\x01\x12\
    \x03\x1f\x06\x12\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x1f\x14\x1b\n\x0c\
    \n\x05\x06\0\x02\x03\x03\x12\x03\x1f&/b\x06proto3\
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
