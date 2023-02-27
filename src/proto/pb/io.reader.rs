// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// length is the request in bytes
    #[prost(int32, tag="1")]
    pub length: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    /// read is the payload in bytes
    #[prost(bytes="bytes", tag="1")]
    pub read: ::prost::bytes::Bytes,
    /// error is an error message
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `io.reader` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x95, 0x06, 0x0a, 0x16, 0x69, 0x6f, 0x2f, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2f, 0x72,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09, 0x69, 0x6f, 0x2e,
    0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x25, 0x0a, 0x0b, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x22, 0x47, 0x0a,
    0x0c, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x72, 0x65, 0x61, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x72, 0x65, 0x61,
    0x64, 0x12, 0x19, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x88, 0x01, 0x01, 0x42, 0x08, 0x0a, 0x06,
    0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x32, 0x41, 0x0a, 0x06, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x37, 0x0a, 0x04, 0x52, 0x65, 0x61, 0x64, 0x12, 0x16, 0x2e, 0x69, 0x6f, 0x2e, 0x72, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x17, 0x2e, 0x69, 0x6f, 0x2e, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x61,
    0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x34, 0x5a, 0x32, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x76, 0x61, 0x2d, 0x6c, 0x61, 0x62, 0x73,
    0x2f, 0x61, 0x76, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x68, 0x65, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2f, 0x70, 0x62, 0x2f, 0x69, 0x6f, 0x2f, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4a,
    0xfe, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x49, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x04, 0x00, 0x49, 0x0a, 0x46, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x1a,
    0x3a, 0x20, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6f, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x73, 0x65, 0x65, 0x3a, 0x20, 0x68, 0x74,
    0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x70, 0x6b, 0x67, 0x2e, 0x67, 0x6f, 0x2e, 0x64, 0x65, 0x76,
    0x2f, 0x69, 0x6f, 0x23, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x08, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x0b, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x21, 0x2d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0b, 0x08, 0x13, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0d, 0x02, 0x13, 0x1a, 0x20, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x11, 0x12, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x14, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x12, 0x02, 0x11, 0x1a, 0x1e, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x79,
    0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x0f, 0x10, 0x0a, 0x28,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x14, 0x02, 0x1c, 0x1a, 0x1b, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x1a, 0x1b,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("io.reader.tonic.rs");
// @@protoc_insertion_point(module)