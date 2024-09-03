/// Originating from an AppFlowy Client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientOrigin {
  /// User id.
  #[prost(int64, tag = "1")]
  pub uid: i64,
  /// Device id.
  #[prost(string, tag = "2")]
  pub device_id: ::prost::alloc::string::String,
}
/// Unknown origin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyOrigin {}
/// Originating from the AppFlowy Server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerOrigin {}
/// Origin of a collab message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabOrigin {
  #[prost(oneof = "collab_origin::Origin", tags = "1, 2, 3")]
  pub origin: ::core::option::Option<collab_origin::Origin>,
}
/// Nested message and enum types in `CollabOrigin`.
pub mod collab_origin {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Origin {
    #[prost(message, tag = "1")]
    Empty(super::EmptyOrigin),
    #[prost(message, tag = "2")]
    Client(super::ClientOrigin),
    #[prost(message, tag = "3")]
    Server(super::ServerOrigin),
  }
}
/// Collab Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CollabType {
  Unknown = 0,
  Document = 1,
  Database = 2,
  WorkspaceDatabase = 3,
  Folder = 4,
  DatabaseRow = 5,
  UserAwareness = 6,
}
impl CollabType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      CollabType::Unknown => "COLLAB_TYPE_UNKNOWN",
      CollabType::Document => "COLLAB_TYPE_DOCUMENT",
      CollabType::Database => "COLLAB_TYPE_DATABASE",
      CollabType::WorkspaceDatabase => "COLLAB_TYPE_WORKSPACE_DATABASE",
      CollabType::Folder => "COLLAB_TYPE_FOLDER",
      CollabType::DatabaseRow => "COLLAB_TYPE_DATABASE_ROW",
      CollabType::UserAwareness => "COLLAB_TYPE_USER_AWARENESS",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "COLLAB_TYPE_UNKNOWN" => Some(Self::Unknown),
      "COLLAB_TYPE_DOCUMENT" => Some(Self::Document),
      "COLLAB_TYPE_DATABASE" => Some(Self::Database),
      "COLLAB_TYPE_WORKSPACE_DATABASE" => Some(Self::WorkspaceDatabase),
      "COLLAB_TYPE_FOLDER" => Some(Self::Folder),
      "COLLAB_TYPE_DATABASE_ROW" => Some(Self::DatabaseRow),
      "COLLAB_TYPE_USER_AWARENESS" => Some(Self::UserAwareness),
      _ => None,
    }
  }
}
/// Encoded collaborative document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncodedCollab {
  /// yrs state vector
  #[prost(bytes = "vec", tag = "1")]
  pub state_vector: ::prost::alloc::vec::Vec<u8>,
  /// yrs document state
  #[prost(bytes = "vec", tag = "2")]
  pub doc_state: ::prost::alloc::vec::Vec<u8>,
  /// yrs encoder version used for the state vector and doc state
  #[prost(enumeration = "EncoderVersion", tag = "3")]
  pub encoder_version: i32,
}
/// yrs encoder version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncoderVersion {
  Unknown = 0,
  V1 = 1,
  V2 = 2,
}
impl EncoderVersion {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      EncoderVersion::Unknown => "ENCODER_VERSION_UNKNOWN",
      EncoderVersion::V1 => "ENCODER_VERSION_V1",
      EncoderVersion::V2 => "ENCODER_VERSION_V2",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ENCODER_VERSION_UNKNOWN" => Some(Self::Unknown),
      "ENCODER_VERSION_V1" => Some(Self::V1),
      "ENCODER_VERSION_V2" => Some(Self::V2),
      _ => None,
    }
  }
}
/// Embeddings and the associated collab metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabEmbeddingsParams {
  /// Fragment id.
  #[prost(string, tag = "1")]
  pub fragment_id: ::prost::alloc::string::String,
  /// Collab object id.
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// Collab type.
  #[prost(enumeration = "CollabType", tag = "3")]
  pub collab_type: i32,
  /// Embedding content type.
  #[prost(enumeration = "EmbeddingContentType", tag = "4")]
  pub content_type: i32,
  /// Embedding content as string.
  #[prost(string, tag = "5")]
  pub content: ::prost::alloc::string::String,
  /// Embedding as float array.
  #[prost(float, repeated, tag = "6")]
  pub embedding: ::prost::alloc::vec::Vec<f32>,
}
/// Wrapper over a collection of embeddings, together with metadata associated on the collection level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabEmbeddings {
  /// OpenAPI tokens consumed.
  #[prost(uint32, tag = "1")]
  pub tokens_consumed: u32,
  /// List of embeddings.
  #[prost(message, repeated, tag = "2")]
  pub embeddings: ::prost::alloc::vec::Vec<CollabEmbeddingsParams>,
}
/// Payload for sending and receive collab over http.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabParams {
  #[prost(string, tag = "1")]
  pub object_id: ::prost::alloc::string::String,
  /// Serialized EncodedCollab object, which could either be in bincode or protobuf serialization format.
  #[prost(bytes = "vec", tag = "2")]
  pub encoded_collab: ::prost::alloc::vec::Vec<u8>,
  /// Collab type.
  #[prost(enumeration = "CollabType", tag = "3")]
  pub collab_type: i32,
  /// Document embeddings.
  #[prost(message, optional, tag = "4")]
  pub embeddings: ::core::option::Option<CollabEmbeddings>,
}
/// Payload for creating batch of collab over http.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateCollabParams {
  /// Workspace id.
  #[prost(string, tag = "1")]
  pub workspace_id: ::prost::alloc::string::String,
  /// List of collab params.
  #[prost(message, repeated, tag = "2")]
  pub params_list: ::prost::alloc::vec::Vec<CollabParams>,
}
/// Payload for creating new collab or update existing collab over http.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollabParams {
  /// Workspace id.
  #[prost(string, tag = "1")]
  pub workspace_id: ::prost::alloc::string::String,
  /// Object id.
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// Serialized EncodedCollab object, which could either be in bincode or protobuf serialization format.
  #[prost(bytes = "vec", tag = "3")]
  pub encoded_collab: ::prost::alloc::vec::Vec<u8>,
  /// Collab type.
  #[prost(enumeration = "CollabType", tag = "4")]
  pub collab_type: i32,
}
/// Types of embeddings content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmbeddingContentType {
  /// Unknown content type.
  Unknown = 0,
  /// Plain text
  PlainText = 1,
}
impl EmbeddingContentType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      EmbeddingContentType::Unknown => "EMBEDDING_CONTENT_TYPE_UNKNOWN",
      EmbeddingContentType::PlainText => "EMBEDDING_CONTENT_TYPE_PLAIN_TEXT",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "EMBEDDING_CONTENT_TYPE_UNKNOWN" => Some(Self::Unknown),
      "EMBEDDING_CONTENT_TYPE_PLAIN_TEXT" => Some(Self::PlainText),
      _ => None,
    }
  }
}
/// Message sent when the origin attempt to sync the payload with a collab document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitSync {
  /// Message origin.
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  /// Object id for the collab.
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// Collab type.
  #[prost(enumeration = "CollabType", tag = "3")]
  pub collab_type: i32,
  /// Workspace which the collab belongs to.
  #[prost(string, tag = "4")]
  pub workspace_id: ::prost::alloc::string::String,
  /// Message id for the sync.
  #[prost(uint64, tag = "5")]
  pub msg_id: u64,
  /// Encoded yrs document state vector.
  #[prost(bytes = "vec", tag = "6")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Update message sent from the origin to the collab.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSync {
  /// Message origin.
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  /// Object id for the collab.
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// Message id for the sync.
  #[prost(uint64, tag = "3")]
  pub msg_id: u64,
  /// Encoded yrs updates.
  #[prost(bytes = "vec", tag = "4")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Metadata for ack message, to be deprecated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckMeta {
  #[prost(string, tag = "1")]
  pub data: ::prost::alloc::string::String,
  #[prost(uint64, tag = "2")]
  pub msg_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabAck {
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// deprecated
  #[prost(message, optional, tag = "3")]
  pub meta: ::core::option::Option<AckMeta>,
  #[prost(bytes = "vec", tag = "4")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
  #[prost(uint32, tag = "5")]
  pub code: u32,
  #[prost(uint64, tag = "6")]
  pub msg_id: u64,
  #[prost(uint32, tag = "7")]
  pub seq_num: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInit {
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  #[prost(uint64, tag = "3")]
  pub msg_id: u64,
  #[prost(bytes = "vec", tag = "4")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwarenessSync {
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  #[prost(bytes = "vec", tag = "3")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastSync {
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  #[prost(bytes = "vec", tag = "3")]
  pub payload: ::prost::alloc::vec::Vec<u8>,
  #[prost(uint32, tag = "4")]
  pub seq_num: u32,
}
/// Wrapper for init sync, for the case when the client is the origin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInitSync {
  #[prost(message, optional, tag = "1")]
  pub data: ::core::option::Option<InitSync>,
}
/// Wrapper for update sync, for the case when the client is the origin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateSync {
  #[prost(message, optional, tag = "1")]
  pub data: ::core::option::Option<UpdateSync>,
}
/// Identifier of an active collab document sent over pubsub
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveCollabId {
  /// Workspace id the active collab belongs to.
  #[prost(string, tag = "1")]
  pub workspace_id: ::prost::alloc::string::String,
  /// Object id
  #[prost(string, tag = "2")]
  pub oid: ::prost::alloc::string::String,
}
/// Update content sent over pubsub
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabUpdateEvent {
  #[prost(oneof = "collab_update_event::Update", tags = "1")]
  pub update: ::core::option::Option<collab_update_event::Update>,
}
/// Nested message and enum types in `CollabUpdateEvent`.
pub mod collab_update_event {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Update {
    /// yrs update in encoded form v1
    #[prost(bytes, tag = "1")]
    UpdateV1(::prost::alloc::vec::Vec<u8>),
  }
}
/// Message sent over websocket to perform collab sync.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabMessage {
  #[prost(oneof = "collab_message::Message", tags = "1, 2, 3, 4, 5, 6")]
  pub message: ::core::option::Option<collab_message::Message>,
}
/// Nested message and enum types in `CollabMessage`.
pub mod collab_message {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Message {
    /// Initial sync on connection, originated from the client.
    #[prost(message, tag = "1")]
    ClientInitSync(super::InitSync),
    /// Document update sync sent by the client.
    #[prost(message, tag = "2")]
    ClientUpdateSync(super::UpdateSync),
    /// Client acknowledgement that the message has been received.
    #[prost(message, tag = "3")]
    ClientAck(super::CollabAck),
    /// Initial sync on connection, originated from the server.
    #[prost(message, tag = "4")]
    ServerInitSync(super::ServerInit),
    /// Awareness update.
    #[prost(message, tag = "5")]
    AwarenessSync(super::AwarenessSync),
    /// Update broadcast from the server to all subscribed clients.
    #[prost(message, tag = "6")]
    ServerBroadcast(super::BroadcastSync),
  }
}
/// User profile change event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChange {
  /// User id.
  #[prost(uint64, tag = "1")]
  pub uid: u64,
  /// User name.
  #[prost(string, optional, tag = "2")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
  /// User email.
  #[prost(string, optional, tag = "3")]
  pub email: ::core::option::Option<::prost::alloc::string::String>,
  /// Metadata.
  #[prost(string, optional, tag = "4")]
  pub metadata: ::core::option::Option<::prost::alloc::string::String>,
}
/// Workspace member.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkspaceMember {
  /// Member's user name
  #[prost(string, tag = "1")]
  pub name: ::prost::alloc::string::String,
  /// Member's email
  #[prost(string, tag = "2")]
  pub email: ::prost::alloc::string::String,
  /// Member's role
  #[prost(enumeration = "Role", tag = "3")]
  pub role: i32,
  /// Member's avatar URL
  #[prost(string, optional, tag = "4")]
  pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
}
/// Workspace member change event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkspaceMemberChange {
  /// List of new members added to the workspace
  #[prost(message, repeated, tag = "1")]
  pub added: ::prost::alloc::vec::Vec<WorkspaceMember>,
  /// List of members in the workspace with updated profile
  #[prost(message, repeated, tag = "2")]
  pub updated: ::prost::alloc::vec::Vec<WorkspaceMember>,
  /// List of members removed from the workspace
  #[prost(message, repeated, tag = "3")]
  pub removed: ::prost::alloc::vec::Vec<WorkspaceMember>,
}
/// Notification on workspace members and user profile related changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserMessage {
  #[prost(oneof = "user_message::Message", tags = "1, 2")]
  pub message: ::core::option::Option<user_message::Message>,
}
/// Nested message and enum types in `UserMessage`.
pub mod user_message {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Message {
    /// User profile change event.
    #[prost(message, tag = "1")]
    ProfileChange(super::UserChange),
    /// Workspace member change event.
    #[prost(message, tag = "2")]
    WorkspaceMemberChange(super::WorkspaceMemberChange),
  }
}
/// Rate limit exceeded notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
  #[prost(uint32, tag = "1")]
  pub limit: u32,
}
/// Connection kick off from the server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KickOff {}
/// There's already an existing websocket connection from the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DuplicateConnection {}
/// Notification on system related events, such as rate limit exceeded and duplicated connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMessage {
  #[prost(oneof = "system_message::Message", tags = "1, 2, 3")]
  pub message: ::core::option::Option<system_message::Message>,
}
/// Nested message and enum types in `SystemMessage`.
pub mod system_message {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Message {
    /// Rate limit exceeded notification.
    #[prost(message, tag = "1")]
    RateLimit(super::RateLimit),
    /// Kick off notification.
    #[prost(message, tag = "2")]
    KickOff(super::KickOff),
    /// There's already an existing websocket connection from the client.
    #[prost(message, tag = "3")]
    DuplicateConnection(super::DuplicateConnection),
  }
}
/// Periodic collab state checker. Not implemented at the moment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabStateCheck {
  /// Collab origin.
  #[prost(message, optional, tag = "1")]
  pub origin: ::core::option::Option<CollabOrigin>,
  /// Object id.
  #[prost(string, tag = "2")]
  pub object_id: ::prost::alloc::string::String,
  /// Message id.
  #[prost(uint64, tag = "3")]
  pub msg_id: u64,
}
/// Collab message originating from the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCollabMessage {
  #[prost(oneof = "client_collab_message::Message", tags = "1, 2, 3, 4, 5")]
  pub message: ::core::option::Option<client_collab_message::Message>,
}
/// Nested message and enum types in `ClientCollabMessage`.
pub mod client_collab_message {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Message {
    /// Initial sync on connection.
    #[prost(message, tag = "1")]
    ClientInitSync(super::ClientInitSync),
    /// Document update sync.
    #[prost(message, tag = "2")]
    ClientUpdateSync(super::ClientUpdateSync),
    /// Updates received from the server during initial sync.
    #[prost(message, tag = "3")]
    ServerInitSync(super::ServerInit),
    /// Awareness update sync.
    #[prost(message, tag = "4")]
    ClientAwarenessSync(super::UpdateSync),
    /// Periodic collab state checker.
    #[prost(message, tag = "5")]
    ClientCollabStateCheck(super::CollabStateCheck),
  }
}
/// Client collab messages in the form of list. To be deprecated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCollabMessageCollectionV1 {
  #[prost(message, repeated, tag = "1")]
  pub messages: ::prost::alloc::vec::Vec<ClientCollabMessage>,
}
/// Client collab messages in the form of map, with the object id as key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCollabMessageCollectionV2 {
  #[prost(map = "string, message", tag = "1")]
  pub messages: ::std::collections::HashMap<::prost::alloc::string::String, ClientCollabMessage>,
}
/// Server collab messages in the form of list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCollabMessageCollection {
  #[prost(message, repeated, tag = "1")]
  pub messages: ::prost::alloc::vec::Vec<CollabMessage>,
}
/// Realtime message sent over websocket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealtimeMessage {
  #[prost(oneof = "realtime_message::Message", tags = "1, 2, 3, 4, 5, 6")]
  pub message: ::core::option::Option<realtime_message::Message>,
}
/// Nested message and enum types in `RealtimeMessage`.
pub mod realtime_message {
  #[allow(clippy::derive_partial_eq_without_eq)]
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Message {
    /// Collab message to perform document and awareness sync.
    #[prost(message, tag = "1")]
    Collab(super::CollabMessage),
    /// Notification on workspace members and user profile related changes.
    #[prost(message, tag = "2")]
    User(super::UserMessage),
    /// Notification on system related events, such as rate limit exceeded and duplicated connection.
    #[prost(message, tag = "3")]
    System(super::SystemMessage),
    /// Collab messages in the form of list. To be deprecated.
    #[prost(message, tag = "4")]
    ClientCollabV1(super::ClientCollabMessageCollectionV1),
    /// Collab messages in the form of map, with the object id as key.
    #[prost(message, tag = "5")]
    ClientCollabV2(super::ClientCollabMessageCollectionV2),
    #[prost(message, tag = "6")]
    Server(super::ServerCollabMessageCollection),
  }
}
/// Workspace member role.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Role {
  /// Unknown role.
  Unknown = 0,
  /// Owner role.
  Owner = 1,
  /// Member role.
  Member = 2,
  /// Guest role.
  Guest = 3,
}
impl Role {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Role::Unknown => "ROLE_UNKNOWN",
      Role::Owner => "ROLE_OWNER",
      Role::Member => "ROLE_MEMBER",
      Role::Guest => "ROLE_GUEST",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ROLE_UNKNOWN" => Some(Self::Unknown),
      "ROLE_OWNER" => Some(Self::Owner),
      "ROLE_MEMBER" => Some(Self::Member),
      "ROLE_GUEST" => Some(Self::Guest),
      _ => None,
    }
  }
}
