// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
/// ModuleRouter defines a route encapsulating pool type.
/// It is used as the value of a mapping from pool id to the pool type,
/// allowing the pool manager to know which module to route swaps to given the
/// pool id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleRoute {
    /// pool_type specifies the type of the pool
    #[prost(enumeration = "PoolType", tag = "1")]
    pub pool_type: i32,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
/// PoolType is an enumeration of all supported pool types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PoolType {
    /// Balancer is the standard xy=k curve. Its pool model is defined in x/gamm.
    Balancer = 0,
    /// Stableswap is the Solidly cfmm stable swap curve. Its pool model is defined
    /// in x/gamm.
    Stableswap = 1,
}
impl PoolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolType::Balancer => "Balancer",
            PoolType::Stableswap => "Stableswap",
        }
    }
}
/// Params holds parameters for the poolmanager module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the poolmanager module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// the next_pool_id
    #[prost(uint64, tag = "1")]
    pub next_pool_id: u64,
    /// params is the container of poolmanager parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// pool_routes is the container of the mappings from pool id to pool type.
    #[prost(message, repeated, tag = "3")]
    pub pool_routes: ::prost::alloc::vec::Vec<ModuleRoute>,
}
/// ===================== MsgSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== Params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// =============================== EstimateSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountInRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountOutRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== NumPools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumPoolsResponse {
    #[prost(uint64, tag = "1")]
    pub num_pools: u64,
}
include!("osmosis.poolmanager.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
