// @generated
/// QuerySpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// QuerySpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
include!("osmosis.gamm.v2.tonic.rs");
// @@protoc_insertion_point(module)
