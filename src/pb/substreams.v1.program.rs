// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub initialize_candidate_list: ::prost::alloc::vec::Vec<InitializeCandidate>,
    #[prost(message, repeated, tag="2")]
    pub initialize_poll_list: ::prost::alloc::vec::Vec<InitializePoll>,
    #[prost(message, repeated, tag="3")]
    pub vote_list: ::prost::alloc::vec::Vec<Vote>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeCandidate {
    #[prost(string, tag="1")]
    pub candidate_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub poll_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePoll {
    #[prost(uint64, tag="1")]
    pub poll_id: u64,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub poll_start: u64,
    #[prost(uint64, tag="4")]
    pub poll_end: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(string, tag="1")]
    pub candidate_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub poll_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    #[prost(string, tag="1")]
    pub candidate_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub candidate_votes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Poll {
    #[prost(uint64, tag="1")]
    pub poll_id: u64,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub poll_start: u64,
    #[prost(uint64, tag="4")]
    pub poll_end: u64,
    #[prost(uint64, tag="5")]
    pub candidate_amount: u64,
}
// @@protoc_insertion_point(module)
