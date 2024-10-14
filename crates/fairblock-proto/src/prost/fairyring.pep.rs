// @generated
/// AggregatedKeyShare defines the structure to store
/// the aggregated keyshare of a particular identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedKeyShare {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
/// EncryptedTx defines the structure to store an encrypted transaction
/// by execution height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTx {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "6")]
    pub processed_at_chain_height: u64,
    #[prost(bool, tag = "7")]
    pub expired: bool,
}
/// EncryptedTxArray defines a list of EncryptedTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx: ::prost::alloc::vec::Vec<EncryptedTx>,
}
/// GeneralEncryptedTx defines the structure to store a
/// general encrypted transaction by identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GeneralEncryptedTxArray defines a list of GeneralEncryptedTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx: ::prost::alloc::vec::Vec<GeneralEncryptedTx>,
}
/// IdentityExecutionQueue defines the structure to queue up
/// identities that have aggregated keyshares available and
/// are ready to execute any associated contracts or encrypted transactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityExecutionQueue {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub tx_list: ::core::option::Option<GeneralEncryptedTxArray>,
    #[prost(string, tag = "6")]
    pub aggr_keyshare: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// option (gogoproto.equal) = true;
    #[prost(string, tag = "1")]
    pub keyshare_channel_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_source_chain: bool,
    #[prost(message, repeated, tag = "3")]
    pub trusted_counter_parties: ::prost::alloc::vec::Vec<TrustedCounterParty>,
    #[prost(string, repeated, tag = "4")]
    pub trusted_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub min_gas_price: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub private_keyshare_price:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// TrustedCounterParty defines the structure to store the ibc info
/// of the source chain (fairyring) to reliably fetch active keys and
/// aggregated/encrypted keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedCounterParty {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
}
/// PepNonce defines the nonce of an account to send encrypted transactions.
/// It is incremanted seperately from the nonce maintained by the auth module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PepNonce {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// RequestId defines the structure for storing request ids
/// that have already been registered to prevent overlap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestId {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
/// PrivateRequest defines the structure for storing private
/// keyshare requests along with the unaggregated encrypted keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<super::common::EncryptedKeyshare>,
}
/// ContractDetails defines the structure to store the details of a
/// contract that has been registered to execute automatically when
/// the identity associated with it has an aggregate keyshare available
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractDetails {
    #[prost(string, tag = "1")]
    pub registrar: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// RegisteredContract defines the structure to store the list of
/// contracts that have been registered with a particular identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredContract {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub contracts: ::prost::alloc::vec::Vec<ContractDetails>,
}
/// ExecuteContractMsg defines the structure to callback registered contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteContractMsg {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub aggr_keyshare: ::prost::alloc::string::String,
}
/// GenesisState defines the pep module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, repeated, tag = "4")]
    pub pep_nonce_list: ::prost::alloc::vec::Vec<PepNonce>,
    #[prost(message, repeated, tag = "6")]
    pub aggregated_key_share_list: ::prost::alloc::vec::Vec<AggregatedKeyShare>,
    #[prost(message, optional, tag = "7")]
    pub active_pub_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "8")]
    pub queued_pub_key: ::core::option::Option<super::common::QueuedPublicKey>,
    #[prost(uint64, tag = "9")]
    pub request_count: u64,
    #[prost(message, repeated, tag = "10")]
    pub request_id_list: ::prost::alloc::vec::Vec<RequestId>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryKeyshareReqRequest is request type for the Query/KeyshareReq RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareReqRequest {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
/// QueryKeyshareReqResponse is response type for the Query/KeyshareReq RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareReqResponse {
    #[prost(message, optional, tag = "1")]
    pub keyshare: ::core::option::Option<IdentityExecutionQueue>,
}
/// QueryKeyshareReqAllRequest is request type for the Query/KeyshareReqAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareReqAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryKeyshareReqAllResponse is response type for the Query/KeyshareReqAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareReqAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub keyshares: ::prost::alloc::vec::Vec<IdentityExecutionQueue>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryEncryptedTxRequest is request type for the Query/EncryptedTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
/// QueryEncryptedTxResponse is response type for the Query/EncryptedTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx: ::core::option::Option<EncryptedTx>,
}
/// QueryEncryptedTxAllRequest is request type for the Query/EncryptedTxAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryEncryptedTxAllResponse is response type for the Query/EncryptedTxAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryEncryptedTxAllFromHeightRequest is request type for the Query/EncryptedTxAllFromHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllFromHeightRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
}
/// QueryEncryptedTxAllFromHeightResponse is response type for the Query/EncryptedTxAllFromHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllFromHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx_array: ::core::option::Option<EncryptedTxArray>,
}
/// QueryLatestHeightRequest is request type for the Query/LatestHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightRequest {}
/// QueryLatestHeightResponse is response type for the Query/LatestHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
/// QueryPepNonceRequest is request type for the Query/PepNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryPepNonceResponse is response type for the Query/PepNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub pep_nonce: ::core::option::Option<PepNonce>,
}
/// QueryPepNonceAllRequest is request type for the Query/PepNonceAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPepNonceAllResponse is response type for the Query/PepNonceAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub pep_nonce: ::prost::alloc::vec::Vec<PepNonce>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPubKeyRequest is request type for the Query/PubKey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubKeyRequest {}
/// QueryPubKeyResponse is response type for the Query/PubKey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub active_pub_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_pub_key: ::core::option::Option<super::common::QueuedPublicKey>,
}
/// QueryPrivateKeyshareReqRequest is request type for the Query/PrivateKeyshareReq RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrivateKeyshareReqRequest {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
/// QueryPrivateKeyshareReqResponse is response type for the Query/PrivateKeyshareReq RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrivateKeyshareReqResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<super::common::EncryptedKeyshare>,
}
/// QueryDecryptDataRequest is request type for the Query/DecryptData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptDataRequest {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub aggr_keyshare: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encrypted_data: ::prost::alloc::string::String,
}
/// QueryDecryptDataResponse is response type for the Query/DecryptData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptDataResponse {
    #[prost(string, tag = "1")]
    pub decrypted_data: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    // params defines the module parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// MsgSubmitEncryptedTx is the Msg/SubmitEncryptedTx request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub target_block_height: u64,
}
/// MsgSubmitEncryptedTxResponse defines the response structure for executing a
/// MsgSubmitEncryptedTx message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTxResponse {}
/// MsgSubmitGeneralEncryptedTx is the Msg/SubmitGeneralEncryptedTx request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgSubmitGeneralEncryptedTxResponse defines the response structure for executing a
/// MsgSubmitGeneralEncryptedTx message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralEncryptedTxResponse {}
/// MsgCreateAggregatedKeyShare is the Msg/CreateAggregatedKeyShare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAggregatedKeyShare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
}
/// MsgCreateAggregatedKeyShareResponse defines the response structure for executing a
/// MsgCreateAggregatedKeyShare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAggregatedKeyShareResponse {}
/// MsgRequestGeneralKeyshare is the Msg/RequestGeneralKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "3")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgRequestGeneralKeyshareResponse defines the response structure for executing a
/// MsgRequestGeneralKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralKeyshareResponse {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgGetGeneralKeyshare is the Msg/GetGeneralKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetGeneralKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgGetGeneralKeyshareResponse defines the response structure for executing a
/// MsgGetGeneralKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetGeneralKeyshareResponse {}
/// MsgRequestPrivateIdentity is the Msg/RequestPrivateIdentity request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateIdentity {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgRequestPrivateIdentityResponse defines the response structure for executing a
/// MsgRequestPrivateIdentity message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateIdentityResponse {
    #[prost(string, tag = "1")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgGetPrivateKeyshares is the Msg/GetPrivateKeyshares request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetPrivateKeyshares {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// MsgGetPrivateKeysharesResponse defines the response structure for executing a
/// MsgGetPrivateKeyshares message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetPrivateKeysharesResponse {}
/// MsgRegisterContract is the Msg/RegisterContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContract {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgRegisterContractResponse defines the response structure for executing a
/// MsgRegisterContract message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContractResponse {}
/// MsgUnregisterContract is the Msg/UnregisterContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterContract {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgUnregisterContractResponse defines the response structure for executing a
/// MsgUnregisterContract message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterContractResponse {}
#[cfg(feature = "grpc")]
include!("fairyring.pep.tonic.rs");
// @@protoc_insertion_point(module)

