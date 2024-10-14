// @generated
/// AggregatedKeyShare defines the structure and height for an aggregated keyshare
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedKeyShare {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// AuthorizedAddress defines if an address is authorized to submit pubkeys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_authorized: bool,
    #[prost(string, tag = "3")]
    pub authorized_by: ::prost::alloc::string::String,
}
/// Commitments defines the list of commitments to verify the
/// keyshares submitted by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitments {
    #[prost(string, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub key_expiry: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_bonded: u64,
    #[prost(uint64, tag = "3")]
    pub max_idled_block: u64,
    #[prost(string, repeated, tag = "4")]
    pub trusted_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "5")]
    pub slash_fraction_no_keyshare: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub slash_fraction_wrong_keyshare: ::prost::alloc::vec::Vec<u8>,
}
/// ValidatorSet defines the structure for storing the list of
/// validators who will be eligible to send keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(string, tag = "1")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cons_addr: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_active: bool,
}
/// KeyShare defines the structure for submitting
/// blockwise keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyShare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub key_share: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
}
/// GeneralKeyShare defines the structure for submitting
/// general keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralKeyShare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key_share: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "7")]
    pub received_block_height: u64,
}
/// ValidatorEncryptedKeyShare defines the structure for
/// submitting encrypted keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorEncryptedKeyShare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key_share: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(string, tag = "7")]
    pub identity: ::prost::alloc::string::String,
}
/// EncryptedKeyShare defines the structure for storing
/// blockwise keyshares submitted by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedKeyShare {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator: ::prost::alloc::string::String,
}
/// ActivePubKey defines the structure of the active public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePubKey {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub expiry: u64,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_key_shares: ::prost::alloc::vec::Vec<EncryptedKeyShare>,
}
/// QueuedPubKey defines the structure of the queued public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedPubKey {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub expiry: u64,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_key_shares: ::prost::alloc::vec::Vec<EncryptedKeyShare>,
}
/// GenesisState defines the keyshare module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub validator_set_list: ::prost::alloc::vec::Vec<ValidatorSet>,
    #[prost(message, repeated, tag = "4")]
    pub key_share_list: ::prost::alloc::vec::Vec<KeyShare>,
    #[prost(message, repeated, tag = "5")]
    pub aggregated_key_share_list: ::prost::alloc::vec::Vec<AggregatedKeyShare>,
    #[prost(message, optional, tag = "6")]
    pub active_pub_key: ::core::option::Option<ActivePubKey>,
    #[prost(message, optional, tag = "7")]
    pub queued_pub_key: ::core::option::Option<QueuedPubKey>,
    #[prost(message, repeated, tag = "8")]
    pub authorized_address_list: ::prost::alloc::vec::Vec<AuthorizedAddress>,
    #[prost(uint64, tag = "9")]
    pub request_count: u64,
    #[prost(message, repeated, tag = "10")]
    pub general_key_share_list: ::prost::alloc::vec::Vec<GeneralKeyShare>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysharePacketData {
    #[prost(
        oneof = "keyshare_packet_data::Packet",
        tags = "1, 2, 3, 4, 5, 6, 7, 8"
    )]
    pub packet: ::core::option::Option<keyshare_packet_data::Packet>,
}
/// Nested message and enum types in `KeysharePacketData`.
pub mod keyshare_packet_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        #[prost(message, tag = "1")]
        NoData(super::NoData),
        #[prost(message, tag = "2")]
        RequestAggrKeysharePacket(super::RequestAggrKeysharePacketData),
        #[prost(message, tag = "3")]
        GetAggrKeysharePacket(super::GetAggrKeysharePacketData),
        #[prost(message, tag = "4")]
        AggrKeyshareDataPacket(super::AggrKeyshareDataPacketData),
        #[prost(message, tag = "5")]
        EncryptedKeysharesPacketData(super::EncryptedKeysharesPacketData),
        #[prost(message, tag = "6")]
        CurrentKeysPacket(super::CurrentKeysPacketData),
        #[prost(message, tag = "7")]
        RequestPrivKeysharePacket(super::RequestPrivateKeysharePacketData),
        #[prost(message, tag = "8")]
        GetPrivateKeysharePacket(super::GetPrivateKeysharePacketData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoData {}
/// RequestAggrKeysharePacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAggrKeysharePacketData {
    #[prost(string, tag = "1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    #[prost(oneof = "request_aggr_keyshare_packet_data::Id", tags = "2, 3")]
    pub id: ::core::option::Option<request_aggr_keyshare_packet_data::Id>,
}
/// Nested message and enum types in `RequestAggrKeysharePacketData`.
pub mod request_aggr_keyshare_packet_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag = "2")]
        ProposalId(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        RequestId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrivateKeysharePacketData {
    #[prost(string, tag = "1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrivateKeysharePacketAck {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// RequestAggrKeysharePacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAggrKeysharePacketAck {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// GetAggrKeysharePacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggrKeysharePacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// GetAggrKeysharePacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggrKeysharePacketAck {}
/// GetPrivateKeysharePacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateKeysharePacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// GetPrivateKeysharePacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateKeysharePacketAck {}
/// AggrKeyshareDataPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggrKeyshareDataPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub aggr_keyshare: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub aggr_height: ::prost::alloc::string::String,
    /// used for private governance
    #[prost(string, tag = "5")]
    pub proposal_id: ::prost::alloc::string::String,
    /// might be useful to destination chains to sort out the response
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub retries: u64,
}
/// AggrKeyshareDataPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggrKeyshareDataPacketAck {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedKeysharesPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<super::common::EncryptedKeyshare>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedKeysharesPacketAck {}
/// CurrentKeysPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentKeysPacketData {}
/// CurrentKeysPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentKeysPacketAck {
    #[prost(message, optional, tag = "1")]
    pub active_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_key: ::core::option::Option<super::common::QueuedPublicKey>,
}
/// QueryVerifiableRandomnessRequest is the request type for
/// the Query/VerifiableRandomness  method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifiableRandomnessRequest {}
/// QueryVerifiableRandomnessResponse is the response type for
/// the Query/VerifiableRandomness  method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifiableRandomnessResponse {
    #[prost(string, tag = "1")]
    pub randomness: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub round: u64,
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
/// QueryCommitmentsRequest is request type for the Query/Commitments RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommitmentsRequest {}
/// QueryCommitmentsResponse is response type for the Query/Commitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommitmentsResponse {
    #[prost(message, optional, tag = "1")]
    pub active_commitments: ::core::option::Option<Commitments>,
    #[prost(message, optional, tag = "2")]
    pub queued_commitments: ::core::option::Option<Commitments>,
}
/// QueryValidatorSetRequest is request type for the Query/ValidatorSet RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetRequest {
    #[prost(string, tag = "1")]
    pub index: ::prost::alloc::string::String,
}
/// QueryValidatorSetResponse is response type for the Query/ValidatorSet RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetResponse {
    #[prost(message, optional, tag = "1")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
/// QueryValidatorSetAllRequest is request type for the Query/ValidatorSetAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorSetAllResponse is response type for the Query/ValidatorSetAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub validator_set: ::prost::alloc::vec::Vec<ValidatorSet>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryKeyShareRequest is request type for the Query/KeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyShareRequest {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
}
/// QueryKeyShareResponse is response type for the Query/KeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyShareResponse {
    #[prost(message, optional, tag = "1")]
    pub key_share: ::core::option::Option<KeyShare>,
}
/// QueryKeyShareAllRequest is request type for the Query/KeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyShareAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryKeyShareAllResponse is response type for the Query/KeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyShareAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub key_share: ::prost::alloc::vec::Vec<KeyShare>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAggregatedKeyShareRequest is request type for the Query/AggregatedKeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatedKeyShareRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
/// QueryAggregatedKeyShareResponse is response type for the Query/AggregatedKeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatedKeyShareResponse {
    #[prost(message, optional, tag = "1")]
    pub aggregated_key_share: ::core::option::Option<AggregatedKeyShare>,
}
/// QueryAggregatedKeyShareAllRequest is request type for the Query/AggregatedKeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatedKeyShareAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAggregatedKeyShareAllResponse is response type for the Query/AggregatedKeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatedKeyShareAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub aggregated_key_share: ::prost::alloc::vec::Vec<AggregatedKeyShare>,
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
    pub active_pub_key: ::core::option::Option<ActivePubKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_pub_key: ::core::option::Option<QueuedPubKey>,
}
/// QueryAuthorizedAddressRequest is request type for the Query/AuthorizedAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressRequest {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
}
/// QueryAuthorizedAddressResponse is response type for the Query/AuthorizedAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub authorized_address: ::core::option::Option<AuthorizedAddress>,
}
/// QueryAuthorizedAddressAllRequest is request type for the Query/AuthorizedAddressAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAuthorizedAddressAllResponse is response type for the Query/AuthorizedAddressAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub authorized_address: ::prost::alloc::vec::Vec<AuthorizedAddress>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGeneralKeyShareRequest is request type for the Query/GeneralKeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyShareRequest {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
}
/// QueryGeneralKeyShareResponse is response type for the Query/GeneralKeyShare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyShareResponse {
    #[prost(message, optional, tag = "1")]
    pub general_key_share: ::core::option::Option<GeneralKeyShare>,
}
/// QueryGeneralKeyShareAllRequest is request type for the Query/GeneralKeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyShareAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGeneralKeyShareAllResponse is response type for the Query/GeneralKeyShareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyShareAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub general_key_share: ::prost::alloc::vec::Vec<GeneralKeyShare>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// KeyShareRequest defines the storage structure for general keyshare requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyShareRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "3")]
    pub ibc_info: ::core::option::Option<IbcInfo>,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<CounterPartyIbcInfo>,
    #[prost(string, tag = "5")]
    pub aggr_keyshare: ::prost::alloc::string::String,
    /// This is only used when the request is for private governance
    #[prost(string, tag = "6")]
    pub proposal_id: ::prost::alloc::string::String,
    /// might be useful to destination chains to sort out the response
    #[prost(string, tag = "7")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub sent: bool,
}
/// IBCInfo defines the structure to verify request for
/// aggregated and encrypted keyshares in case the request was made over IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbcInfo {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
}
/// CounterPartyIBCInfo defines the structure to send aggregated
/// and encrypted keyshares if the request was made over IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterPartyIbcInfo {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
}
/// PrivateKeyshareRequest defines the stroage structure for private
/// encrypted and unaggregated keyshare requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyshareRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "3")]
    pub ibc_info: ::core::option::Option<IbcInfo>,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<CounterPartyIbcInfo>,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<super::common::EncryptedKeyshare>,
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub sent: bool,
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
/// MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// MsgRegisterValidator is the Msg/RegisterValidator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterValidator {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgRegisterValidatorResponse defines the response structure for
/// executing a MsgRegisterValidator message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterValidatorResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeRegisterValidator is the Msg/DeRegisterValidator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeRegisterValidator {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeRegisterValidatorResponse defines the response structure for
/// executing a MsgDeRegisterValidator message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeRegisterValidatorResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgSendKeyshare is the Msg/SendKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
}
/// MsgSendKeyshareResponse defines the response structure for
/// executing a MsgSendKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendKeyshareResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
    #[prost(uint64, tag = "5")]
    pub received_block_height: u64,
    #[prost(bool, tag = "6")]
    pub success: bool,
    #[prost(string, tag = "7")]
    pub error_message: ::prost::alloc::string::String,
}
/// MsgCreateLatestPubKey is the Msg/CreateLatestPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLatestPubKey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_key_shares: ::prost::alloc::vec::Vec<EncryptedKeyShare>,
}
/// MsgCreateLatestPubKeyResponse defines the response structure for
/// executing a MsgCreateLatestPubKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLatestPubKeyResponse {}
/// MsgOverrideLatestPubKey is the Msg/OverrideLatestPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOverrideLatestPubKey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_key_shares: ::prost::alloc::vec::Vec<EncryptedKeyShare>,
}
/// MsgOverrideLatestPubKeyResponse defines the response structure for
/// executing a MsgOverrideLatestPubKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOverrideLatestPubKeyResponse {}
/// MsgCreateAuthorizedAddress is the Msg/CreateAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgCreateAuthorizedAddressResponse defines the response structure for
/// executing a MsgCreateAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuthorizedAddressResponse {}
/// MsgUpdateAuthorizedAddress is the Msg/UpdateAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_authorized: bool,
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgUpdateAuthorizedAddressResponse defines the response structure for
/// executing a MsgUpdateAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAuthorizedAddressResponse {}
/// MsgDeleteAuthorizedAddress is the Msg/DeleteAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeleteAuthorizedAddressResponse defines the response structure for
/// executing a MsgDeleteAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthorizedAddressResponse {}
/// MsgCreateGeneralKeyShare is the Msg/CreateGeneralKeyShare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGeneralKeyShare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key_share: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "7")]
    pub received_block_height: u64,
}
/// MsgCreateGeneralKeyShareResponse defines the response structure for
/// executing a MsgCreateGeneralKeyShare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGeneralKeyShareResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key_share: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(bool, tag = "7")]
    pub success: bool,
    #[prost(string, tag = "8")]
    pub error_message: ::prost::alloc::string::String,
}
/// MsgSubmitEncryptedKeyshare is the Msg/SubmitEncryptedKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encrypted_keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub key_share_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(string, tag = "7")]
    pub requester: ::prost::alloc::string::String,
}
/// MsgSubmitEncryptedKeyshareResponse defines the response structure for
/// executing a MsgSubmitEncryptedKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedKeyshareResponse {}
#[cfg(feature = "grpc")]
include!("fairyring.keyshare.tonic.rs");
// @@protoc_insertion_point(module)

