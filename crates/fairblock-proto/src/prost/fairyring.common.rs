// @generated
/// RequestAggrKeyshare defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAggrKeyshare {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    /// id can either be a rwquest id or a proposal id
    #[prost(oneof="request_aggr_keyshare::Id", tags="2, 3")]
    pub id: ::core::option::Option<request_aggr_keyshare::Id>,
}
/// Nested message and enum types in `RequestAggrKeyshare`.
pub mod request_aggr_keyshare {
    /// id can either be a rwquest id or a proposal id
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag="2")]
        ProposalId(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        RequestId(::prost::alloc::string::String),
    }
}
/// RequestAggrKeyshareResponse defines the response to the RequestAggrKeyshare message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAggrKeyshareResponse {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// GetAggrKeyshare defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggrKeyshare {
    #[prost(string, tag="3")]
    pub identity: ::prost::alloc::string::String,
    /// id can either be a rwquest id or a proposal id
    #[prost(oneof="get_aggr_keyshare::Id", tags="1, 2")]
    pub id: ::core::option::Option<get_aggr_keyshare::Id>,
}
/// Nested message and enum types in `GetAggrKeyshare`.
pub mod get_aggr_keyshare {
    /// id can either be a rwquest id or a proposal id
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag="1")]
        ProposalId(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        RequestId(::prost::alloc::string::String),
    }
}
/// GetAggrKeyshareResponse defines the response to the GetAggrKeyshare message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggrKeyshareResponse {
}
/// GetPrivateKeyshare defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateKeyshare {
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// GetPrivateKeyshareResponse defines the response to the GetPrivateKeyshare message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateKeyshareResponse {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
}
/// ActivePublicKey defines the pubkey currently in use
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePublicKey {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub expiry: u64,
}
/// QueuedPublicKey defines the pubkey that (when set) will replace the acive pubkey
/// when it expires
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedPublicKey {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub expiry: u64,
}
/// RequestEncryptedKeyshare defines the structure to request for
/// encrypted and unaggregated keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEncryptedKeyshare {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// EncryptedKeyshare defines the storage structure for
/// the list of encrypted keyshares (unaggregated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedKeyshare {
    #[prost(string, tag="1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub private_keyshares: ::prost::alloc::vec::Vec<IndexedEncryptedKeyshare>,
}
/// IndexedEncryptedKeyshare defines the storage of submitted encrypted
/// keyshares along with their indices (can be decrypted and aggregated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedEncryptedKeyshare {
    #[prost(string, tag="1")]
    pub encrypted_keyshare_value: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub encrypted_keyshare_index: u64,
}
// @@protoc_insertion_point(module)
