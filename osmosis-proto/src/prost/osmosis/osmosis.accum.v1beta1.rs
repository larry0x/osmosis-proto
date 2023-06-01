// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccumulatorContent {
    #[prost(message, repeated, tag = "1")]
    pub accum_value: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(string, tag = "2")]
    pub total_shares: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    #[prost(string, tag = "1")]
    pub num_shares: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub init_accum_value:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, repeated, tag = "3")]
    pub unclaimed_rewards:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<Options>,
}
// @@protoc_insertion_point(module)
