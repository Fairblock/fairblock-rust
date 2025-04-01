// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag="1")]
    pub minimum_delegation_amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the auction module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub auction_detail_list: ::prost::alloc::vec::Vec<super::common::AuctionDetailList>,
    #[prost(message, repeated, tag="3")]
    pub registered_bidders: ::prost::alloc::vec::Vec<super::common::Bidder>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionRequest {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionResponse {
    #[prost(message, optional, tag="1")]
    pub auction_detail: ::core::option::Option<super::common::AuctionDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionAllRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionAllResponse {
    #[prost(message, repeated, tag="1")]
    pub auction_details: ::prost::alloc::vec::Vec<super::common::AuctionDetail>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
// message QueryTimedAuctionRequest {
//   string identity = 1;
// }
//
// message QueryTimedAuctionResponse {
//   AuctionDetail auction_detail = 1;
// }
//
// message QueryTimedAuctionAllRequest {
//   cosmos.base.query.v1beta1.PageRequest pagination = 1;
// }
//
// message QueryTimedAuctionAllResponse {
//   repeated AuctionDetail auction_details = 1;
//   cosmos.base.query.v1beta1.PageResponse pagination = 2;
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredBiddersRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredBiddersResponse {
    #[prost(message, repeated, tag="1")]
    pub bidders: ::prost::alloc::vec::Vec<super::common::Bidder>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceBid {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sealed_bid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub auction_identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceBidResponse {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sealed_bid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub auction_identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuction {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub resolve_at: u64,
    #[prost(bool, tag="3")]
    pub is_timed: bool,
    #[prost(string, tag="4")]
    pub bid_denom: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub is_first_price: bool,
    #[prost(bool, tag="6")]
    pub auto_deduct_from_winner: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuctionResponse {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub auction_id: u64,
    #[prost(uint64, tag="4")]
    pub resolve_at: u64,
}
// message MsgCreateTimedAuction {
//   option (cosmos.msg.v1.signer) = "creator";
//   string creator = 1;
//   uint64 resolution_height = 2;
//   string auction_id = 3;
//   bool is_second_price = 4;
// }
//
// message MsgCreateTimedAuctionResponse {
//   string identity = 1;
//   string pubkey = 2;
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResolveAuction {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResolveAuctionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterBidder {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bidder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterBidderResponse {
    #[prost(message, optional, tag="1")]
    pub delegated: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeregisterBidder {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bidder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeregisterBidderResponse {
    #[prost(message, optional, tag="1")]
    pub refunded: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the module parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
#[cfg(feature = "grpc")]
include!("fairyring.auction.tonic.rs");
// @@protoc_insertion_point(module)