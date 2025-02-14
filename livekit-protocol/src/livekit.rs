// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub empty_timeout: u32,
    #[prost(uint32, tag="4")]
    pub max_participants: u32,
    #[prost(int64, tag="5")]
    pub creation_time: i64,
    #[prost(string, tag="6")]
    pub turn_password: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub enabled_codecs: ::prost::alloc::vec::Vec<Codec>,
    #[prost(string, tag="8")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub num_participants: u32,
    #[prost(uint32, tag="11")]
    pub num_publishers: u32,
    #[prost(bool, tag="10")]
    pub active_recording: bool,
    #[prost(message, optional, tag="13")]
    pub version: ::core::option::Option<TimedVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Codec {
    #[prost(string, tag="1")]
    pub mime: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fmtp_line: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayoutDelay {
    #[prost(bool, tag="1")]
    pub enabled: bool,
    #[prost(uint32, tag="2")]
    pub min: u32,
    #[prost(uint32, tag="3")]
    pub max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantPermission {
    /// allow participant to subscribe to other tracks in the room
    #[prost(bool, tag="1")]
    pub can_subscribe: bool,
    /// allow participant to publish new tracks to room
    #[prost(bool, tag="2")]
    pub can_publish: bool,
    /// allow participant to publish data
    #[prost(bool, tag="3")]
    pub can_publish_data: bool,
    /// sources that are allowed to be published
    #[prost(enumeration="TrackSource", repeated, tag="9")]
    pub can_publish_sources: ::prost::alloc::vec::Vec<i32>,
    /// indicates that it's hidden to others
    #[prost(bool, tag="7")]
    pub hidden: bool,
    /// indicates it's a recorder instance
    #[prost(bool, tag="8")]
    pub recorder: bool,
    /// indicates that participant can update own metadata
    #[prost(bool, tag="10")]
    pub can_update_metadata: bool,
    /// indicates that participant is an agent
    #[prost(bool, tag="11")]
    pub agent: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(enumeration="participant_info::State", tag="3")]
    pub state: i32,
    #[prost(message, repeated, tag="4")]
    pub tracks: ::prost::alloc::vec::Vec<TrackInfo>,
    #[prost(string, tag="5")]
    pub metadata: ::prost::alloc::string::String,
    /// timestamp when participant joined room, in seconds
    #[prost(int64, tag="6")]
    pub joined_at: i64,
    #[prost(string, tag="9")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="10")]
    pub version: u32,
    #[prost(message, optional, tag="11")]
    pub permission: ::core::option::Option<ParticipantPermission>,
    #[prost(string, tag="12")]
    pub region: ::prost::alloc::string::String,
    /// indicates the participant has an active publisher connection
    /// and can publish to the server
    #[prost(bool, tag="13")]
    pub is_publisher: bool,
    #[prost(enumeration="participant_info::Kind", tag="14")]
    pub kind: i32,
}
/// Nested message and enum types in `ParticipantInfo`.
pub mod participant_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// websocket' connected, but not offered yet
        Joining = 0,
        /// server received client offer
        Joined = 1,
        /// ICE connectivity established
        Active = 2,
        /// WS disconnected
        Disconnected = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Joining => "JOINING",
                State::Joined => "JOINED",
                State::Active => "ACTIVE",
                State::Disconnected => "DISCONNECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOINING" => Some(Self::Joining),
                "JOINED" => Some(Self::Joined),
                "ACTIVE" => Some(Self::Active),
                "DISCONNECTED" => Some(Self::Disconnected),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// standard participants, e.g. web clients
        Standard = 0,
        /// only ingests streams
        Ingress = 1,
        /// only consumes streams
        Egress = 2,
        /// SIP participants
        Sip = 3,
        /// LiveKit agents
        Agent = 4,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Standard => "STANDARD",
                Kind::Ingress => "INGRESS",
                Kind::Egress => "EGRESS",
                Kind::Sip => "SIP",
                Kind::Agent => "AGENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STANDARD" => Some(Self::Standard),
                "INGRESS" => Some(Self::Ingress),
                "EGRESS" => Some(Self::Egress),
                "SIP" => Some(Self::Sip),
                "AGENT" => Some(Self::Agent),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encryption {
}
/// Nested message and enum types in `Encryption`.
pub mod encryption {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        None = 0,
        Gcm = 1,
        Custom = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Gcm => "GCM",
                Type::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "GCM" => Some(Self::Gcm),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulcastCodecInfo {
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub layers: ::prost::alloc::vec::Vec<VideoLayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(enumeration="TrackType", tag="2")]
    pub r#type: i32,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub muted: bool,
    /// original width of video (unset for audio)
    /// clients may receive a lower resolution version with simulcast
    #[prost(uint32, tag="5")]
    pub width: u32,
    /// original height of video (unset for audio)
    #[prost(uint32, tag="6")]
    pub height: u32,
    /// true if track is simulcasted
    #[prost(bool, tag="7")]
    pub simulcast: bool,
    /// true if DTX (Discontinuous Transmission) is disabled for audio
    #[prost(bool, tag="8")]
    pub disable_dtx: bool,
    /// source of media
    #[prost(enumeration="TrackSource", tag="9")]
    pub source: i32,
    #[prost(message, repeated, tag="10")]
    pub layers: ::prost::alloc::vec::Vec<VideoLayer>,
    /// mime type of codec
    #[prost(string, tag="11")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub mid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="13")]
    pub codecs: ::prost::alloc::vec::Vec<SimulcastCodecInfo>,
    #[prost(bool, tag="14")]
    pub stereo: bool,
    /// true if RED (Redundant Encoding) is disabled for audio
    #[prost(bool, tag="15")]
    pub disable_red: bool,
    #[prost(enumeration="encryption::Type", tag="16")]
    pub encryption: i32,
    #[prost(string, tag="17")]
    pub stream: ::prost::alloc::string::String,
    #[prost(message, optional, tag="18")]
    pub version: ::core::option::Option<TimedVersion>,
}
/// provide information about available spatial layers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoLayer {
    /// for tracks with a single layer, this should be HIGH
    #[prost(enumeration="VideoQuality", tag="1")]
    pub quality: i32,
    #[prost(uint32, tag="2")]
    pub width: u32,
    #[prost(uint32, tag="3")]
    pub height: u32,
    /// target bitrate in bit per second (bps), server will measure actual
    #[prost(uint32, tag="4")]
    pub bitrate: u32,
    #[prost(uint32, tag="5")]
    pub ssrc: u32,
}
/// new DataPacket API
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataPacket {
    #[prost(enumeration="data_packet::Kind", tag="1")]
    pub kind: i32,
    #[prost(oneof="data_packet::Value", tags="2, 3")]
    pub value: ::core::option::Option<data_packet::Value>,
}
/// Nested message and enum types in `DataPacket`.
pub mod data_packet {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Reliable = 0,
        Lossy = 1,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Reliable => "RELIABLE",
                Kind::Lossy => "LOSSY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RELIABLE" => Some(Self::Reliable),
                "LOSSY" => Some(Self::Lossy),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="2")]
        User(super::UserPacket),
        #[prost(message, tag="3")]
        Speaker(super::ActiveSpeakerUpdate),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveSpeakerUpdate {
    #[prost(message, repeated, tag="1")]
    pub speakers: ::prost::alloc::vec::Vec<SpeakerInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeakerInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    /// audio level, 0-1.0, 1 is loudest
    #[prost(float, tag="2")]
    pub level: f32,
    /// true if speaker is currently active
    #[prost(bool, tag="3")]
    pub active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPacket {
    /// participant ID of user that sent the message
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub participant_identity: ::prost::alloc::string::String,
    /// user defined payload
    #[prost(bytes="vec", tag="2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// the ID of the participants who will receive the message (sent to all by default)
    #[prost(string, repeated, tag="3")]
    pub destination_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// identities of participants who will receive the message (sent to all by default)
    #[prost(string, repeated, tag="6")]
    pub destination_identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// topic under which the message was published
    #[prost(string, optional, tag="4")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantTracks {
    /// participant ID of participant to whom the tracks belong
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub track_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// details about the server
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfo {
    #[prost(enumeration="server_info::Edition", tag="1")]
    pub edition: i32,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub protocol: i32,
    #[prost(string, tag="4")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub node_id: ::prost::alloc::string::String,
    /// additional debugging information. sent only if server is in development mode
    #[prost(string, tag="6")]
    pub debug_info: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ServerInfo`.
pub mod server_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Edition {
        Standard = 0,
        Cloud = 1,
    }
    impl Edition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Edition::Standard => "Standard",
                Edition::Cloud => "Cloud",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Standard" => Some(Self::Standard),
                "Cloud" => Some(Self::Cloud),
                _ => None,
            }
        }
    }
}
/// details about the client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    #[prost(enumeration="client_info::Sdk", tag="1")]
    pub sdk: i32,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub protocol: i32,
    #[prost(string, tag="4")]
    pub os: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub os_version: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub device_model: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub browser: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub browser_version: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub address: ::prost::alloc::string::String,
    /// wifi, wired, cellular, vpn, empty if not known
    #[prost(string, tag="10")]
    pub network: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ClientInfo`.
pub mod client_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Sdk {
        Unknown = 0,
        Js = 1,
        Swift = 2,
        Android = 3,
        Flutter = 4,
        Go = 5,
        Unity = 6,
        ReactNative = 7,
        Rust = 8,
        Python = 9,
        Cpp = 10,
    }
    impl Sdk {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Sdk::Unknown => "UNKNOWN",
                Sdk::Js => "JS",
                Sdk::Swift => "SWIFT",
                Sdk::Android => "ANDROID",
                Sdk::Flutter => "FLUTTER",
                Sdk::Go => "GO",
                Sdk::Unity => "UNITY",
                Sdk::ReactNative => "REACT_NATIVE",
                Sdk::Rust => "RUST",
                Sdk::Python => "PYTHON",
                Sdk::Cpp => "CPP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "JS" => Some(Self::Js),
                "SWIFT" => Some(Self::Swift),
                "ANDROID" => Some(Self::Android),
                "FLUTTER" => Some(Self::Flutter),
                "GO" => Some(Self::Go),
                "UNITY" => Some(Self::Unity),
                "REACT_NATIVE" => Some(Self::ReactNative),
                "RUST" => Some(Self::Rust),
                "PYTHON" => Some(Self::Python),
                "CPP" => Some(Self::Cpp),
                _ => None,
            }
        }
    }
}
/// server provided client configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConfiguration {
    #[prost(message, optional, tag="1")]
    pub video: ::core::option::Option<VideoConfiguration>,
    #[prost(message, optional, tag="2")]
    pub screen: ::core::option::Option<VideoConfiguration>,
    #[prost(enumeration="ClientConfigSetting", tag="3")]
    pub resume_connection: i32,
    #[prost(message, optional, tag="4")]
    pub disabled_codecs: ::core::option::Option<DisabledCodecs>,
    #[prost(enumeration="ClientConfigSetting", tag="5")]
    pub force_relay: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoConfiguration {
    #[prost(enumeration="ClientConfigSetting", tag="1")]
    pub hardware_encoder: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisabledCodecs {
    /// disabled for both publish and subscribe
    #[prost(message, repeated, tag="1")]
    pub codecs: ::prost::alloc::vec::Vec<Codec>,
    /// only disable for publish
    #[prost(message, repeated, tag="2")]
    pub publish: ::prost::alloc::vec::Vec<Codec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtpDrift {
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(double, tag="3")]
    pub duration: f64,
    #[prost(uint64, tag="4")]
    pub start_timestamp: u64,
    #[prost(uint64, tag="5")]
    pub end_timestamp: u64,
    #[prost(uint64, tag="6")]
    pub rtp_clock_ticks: u64,
    #[prost(int64, tag="7")]
    pub drift_samples: i64,
    #[prost(double, tag="8")]
    pub drift_ms: f64,
    #[prost(double, tag="9")]
    pub clock_rate: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtpStats {
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(double, tag="3")]
    pub duration: f64,
    #[prost(uint32, tag="4")]
    pub packets: u32,
    #[prost(double, tag="5")]
    pub packet_rate: f64,
    #[prost(uint64, tag="6")]
    pub bytes: u64,
    #[prost(uint64, tag="39")]
    pub header_bytes: u64,
    #[prost(double, tag="7")]
    pub bitrate: f64,
    #[prost(uint32, tag="8")]
    pub packets_lost: u32,
    #[prost(double, tag="9")]
    pub packet_loss_rate: f64,
    #[prost(float, tag="10")]
    pub packet_loss_percentage: f32,
    #[prost(uint32, tag="11")]
    pub packets_duplicate: u32,
    #[prost(double, tag="12")]
    pub packet_duplicate_rate: f64,
    #[prost(uint64, tag="13")]
    pub bytes_duplicate: u64,
    #[prost(uint64, tag="40")]
    pub header_bytes_duplicate: u64,
    #[prost(double, tag="14")]
    pub bitrate_duplicate: f64,
    #[prost(uint32, tag="15")]
    pub packets_padding: u32,
    #[prost(double, tag="16")]
    pub packet_padding_rate: f64,
    #[prost(uint64, tag="17")]
    pub bytes_padding: u64,
    #[prost(uint64, tag="41")]
    pub header_bytes_padding: u64,
    #[prost(double, tag="18")]
    pub bitrate_padding: f64,
    #[prost(uint32, tag="19")]
    pub packets_out_of_order: u32,
    #[prost(uint32, tag="20")]
    pub frames: u32,
    #[prost(double, tag="21")]
    pub frame_rate: f64,
    #[prost(double, tag="22")]
    pub jitter_current: f64,
    #[prost(double, tag="23")]
    pub jitter_max: f64,
    #[prost(map="int32, uint32", tag="24")]
    pub gap_histogram: ::std::collections::HashMap<i32, u32>,
    #[prost(uint32, tag="25")]
    pub nacks: u32,
    #[prost(uint32, tag="37")]
    pub nack_acks: u32,
    #[prost(uint32, tag="26")]
    pub nack_misses: u32,
    #[prost(uint32, tag="38")]
    pub nack_repeated: u32,
    #[prost(uint32, tag="27")]
    pub plis: u32,
    #[prost(message, optional, tag="28")]
    pub last_pli: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="29")]
    pub firs: u32,
    #[prost(message, optional, tag="30")]
    pub last_fir: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="31")]
    pub rtt_current: u32,
    #[prost(uint32, tag="32")]
    pub rtt_max: u32,
    #[prost(uint32, tag="33")]
    pub key_frames: u32,
    #[prost(message, optional, tag="34")]
    pub last_key_frame: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="35")]
    pub layer_lock_plis: u32,
    #[prost(message, optional, tag="36")]
    pub last_layer_lock_pli: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="44")]
    pub packet_drift: ::core::option::Option<RtpDrift>,
    /// NEXT_ID: 46
    #[prost(message, optional, tag="45")]
    pub report_drift: ::core::option::Option<RtpDrift>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimedVersion {
    #[prost(int64, tag="1")]
    pub unix_micro: i64,
    #[prost(int32, tag="2")]
    pub ticks: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioCodec {
    DefaultAc = 0,
    Opus = 1,
    Aac = 2,
}
impl AudioCodec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioCodec::DefaultAc => "DEFAULT_AC",
            AudioCodec::Opus => "OPUS",
            AudioCodec::Aac => "AAC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_AC" => Some(Self::DefaultAc),
            "OPUS" => Some(Self::Opus),
            "AAC" => Some(Self::Aac),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoCodec {
    DefaultVc = 0,
    H264Baseline = 1,
    H264Main = 2,
    H264High = 3,
    Vp8 = 4,
}
impl VideoCodec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoCodec::DefaultVc => "DEFAULT_VC",
            VideoCodec::H264Baseline => "H264_BASELINE",
            VideoCodec::H264Main => "H264_MAIN",
            VideoCodec::H264High => "H264_HIGH",
            VideoCodec::Vp8 => "VP8",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_VC" => Some(Self::DefaultVc),
            "H264_BASELINE" => Some(Self::H264Baseline),
            "H264_MAIN" => Some(Self::H264Main),
            "H264_HIGH" => Some(Self::H264High),
            "VP8" => Some(Self::Vp8),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImageCodec {
    IcDefault = 0,
    IcJpeg = 1,
}
impl ImageCodec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImageCodec::IcDefault => "IC_DEFAULT",
            ImageCodec::IcJpeg => "IC_JPEG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IC_DEFAULT" => Some(Self::IcDefault),
            "IC_JPEG" => Some(Self::IcJpeg),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrackType {
    Audio = 0,
    Video = 1,
    Data = 2,
}
impl TrackType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrackType::Audio => "AUDIO",
            TrackType::Video => "VIDEO",
            TrackType::Data => "DATA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO" => Some(Self::Audio),
            "VIDEO" => Some(Self::Video),
            "DATA" => Some(Self::Data),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrackSource {
    Unknown = 0,
    Camera = 1,
    Microphone = 2,
    ScreenShare = 3,
    ScreenShareAudio = 4,
}
impl TrackSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrackSource::Unknown => "UNKNOWN",
            TrackSource::Camera => "CAMERA",
            TrackSource::Microphone => "MICROPHONE",
            TrackSource::ScreenShare => "SCREEN_SHARE",
            TrackSource::ScreenShareAudio => "SCREEN_SHARE_AUDIO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "CAMERA" => Some(Self::Camera),
            "MICROPHONE" => Some(Self::Microphone),
            "SCREEN_SHARE" => Some(Self::ScreenShare),
            "SCREEN_SHARE_AUDIO" => Some(Self::ScreenShareAudio),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoQuality {
    Low = 0,
    Medium = 1,
    High = 2,
    Off = 3,
}
impl VideoQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoQuality::Low => "LOW",
            VideoQuality::Medium => "MEDIUM",
            VideoQuality::High => "HIGH",
            VideoQuality::Off => "OFF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "MEDIUM" => Some(Self::Medium),
            "HIGH" => Some(Self::High),
            "OFF" => Some(Self::Off),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionQuality {
    Poor = 0,
    Good = 1,
    Excellent = 2,
    Lost = 3,
}
impl ConnectionQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionQuality::Poor => "POOR",
            ConnectionQuality::Good => "GOOD",
            ConnectionQuality::Excellent => "EXCELLENT",
            ConnectionQuality::Lost => "LOST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POOR" => Some(Self::Poor),
            "GOOD" => Some(Self::Good),
            "EXCELLENT" => Some(Self::Excellent),
            "LOST" => Some(Self::Lost),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientConfigSetting {
    Unset = 0,
    Disabled = 1,
    Enabled = 2,
}
impl ClientConfigSetting {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientConfigSetting::Unset => "UNSET",
            ClientConfigSetting::Disabled => "DISABLED",
            ClientConfigSetting::Enabled => "ENABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSET" => Some(Self::Unset),
            "DISABLED" => Some(Self::Disabled),
            "ENABLED" => Some(Self::Enabled),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisconnectReason {
    UnknownReason = 0,
    ClientInitiated = 1,
    DuplicateIdentity = 2,
    ServerShutdown = 3,
    ParticipantRemoved = 4,
    RoomDeleted = 5,
    StateMismatch = 6,
    JoinFailure = 7,
}
impl DisconnectReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DisconnectReason::UnknownReason => "UNKNOWN_REASON",
            DisconnectReason::ClientInitiated => "CLIENT_INITIATED",
            DisconnectReason::DuplicateIdentity => "DUPLICATE_IDENTITY",
            DisconnectReason::ServerShutdown => "SERVER_SHUTDOWN",
            DisconnectReason::ParticipantRemoved => "PARTICIPANT_REMOVED",
            DisconnectReason::RoomDeleted => "ROOM_DELETED",
            DisconnectReason::StateMismatch => "STATE_MISMATCH",
            DisconnectReason::JoinFailure => "JOIN_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_REASON" => Some(Self::UnknownReason),
            "CLIENT_INITIATED" => Some(Self::ClientInitiated),
            "DUPLICATE_IDENTITY" => Some(Self::DuplicateIdentity),
            "SERVER_SHUTDOWN" => Some(Self::ServerShutdown),
            "PARTICIPANT_REMOVED" => Some(Self::ParticipantRemoved),
            "ROOM_DELETED" => Some(Self::RoomDeleted),
            "STATE_MISMATCH" => Some(Self::StateMismatch),
            "JOIN_FAILURE" => Some(Self::JoinFailure),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReconnectReason {
    RrUnknown = 0,
    RrSignalDisconnected = 1,
    RrPublisherFailed = 2,
    RrSubscriberFailed = 3,
    RrSwitchCandidate = 4,
}
impl ReconnectReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReconnectReason::RrUnknown => "RR_UNKNOWN",
            ReconnectReason::RrSignalDisconnected => "RR_SIGNAL_DISCONNECTED",
            ReconnectReason::RrPublisherFailed => "RR_PUBLISHER_FAILED",
            ReconnectReason::RrSubscriberFailed => "RR_SUBSCRIBER_FAILED",
            ReconnectReason::RrSwitchCandidate => "RR_SWITCH_CANDIDATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RR_UNKNOWN" => Some(Self::RrUnknown),
            "RR_SIGNAL_DISCONNECTED" => Some(Self::RrSignalDisconnected),
            "RR_PUBLISHER_FAILED" => Some(Self::RrPublisherFailed),
            "RR_SUBSCRIBER_FAILED" => Some(Self::RrSubscriberFailed),
            "RR_SWITCH_CANDIDATE" => Some(Self::RrSwitchCandidate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionError {
    SeUnknown = 0,
    SeCodecUnsupported = 1,
    SeTrackNotfound = 2,
}
impl SubscriptionError {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionError::SeUnknown => "SE_UNKNOWN",
            SubscriptionError::SeCodecUnsupported => "SE_CODEC_UNSUPPORTED",
            SubscriptionError::SeTrackNotfound => "SE_TRACK_NOTFOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SE_UNKNOWN" => Some(Self::SeUnknown),
            "SE_CODEC_UNSUPPORTED" => Some(Self::SeCodecUnsupported),
            "SE_TRACK_NOTFOUND" => Some(Self::SeTrackNotfound),
            _ => None,
        }
    }
}
/// composite using a web browser
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomCompositeEgressRequest {
    /// required
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// (optional)
    #[prost(string, tag="2")]
    pub layout: ::prost::alloc::string::String,
    /// (default false)
    #[prost(bool, tag="3")]
    pub audio_only: bool,
    /// (default false)
    #[prost(bool, tag="4")]
    pub video_only: bool,
    /// template base url (default <https://recorder.livekit.io>)
    #[prost(string, tag="5")]
    pub custom_base_url: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="11")]
    pub file_outputs: ::prost::alloc::vec::Vec<EncodedFileOutput>,
    #[prost(message, repeated, tag="12")]
    pub stream_outputs: ::prost::alloc::vec::Vec<StreamOutput>,
    #[prost(message, repeated, tag="13")]
    pub segment_outputs: ::prost::alloc::vec::Vec<SegmentedFileOutput>,
    #[prost(message, repeated, tag="14")]
    pub image_outputs: ::prost::alloc::vec::Vec<ImageOutput>,
    /// deprecated (use _output fields)
    #[prost(oneof="room_composite_egress_request::Output", tags="6, 7, 10")]
    pub output: ::core::option::Option<room_composite_egress_request::Output>,
    #[prost(oneof="room_composite_egress_request::Options", tags="8, 9")]
    pub options: ::core::option::Option<room_composite_egress_request::Options>,
}
/// Nested message and enum types in `RoomCompositeEgressRequest`.
pub mod room_composite_egress_request {
    /// deprecated (use _output fields)
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="6")]
        File(super::EncodedFileOutput),
        #[prost(message, tag="7")]
        Stream(super::StreamOutput),
        #[prost(message, tag="10")]
        Segments(super::SegmentedFileOutput),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// (default H264_720P_30)
        #[prost(enumeration="super::EncodingOptionsPreset", tag="8")]
        Preset(i32),
        /// (optional)
        #[prost(message, tag="9")]
        Advanced(super::EncodingOptions),
    }
}
/// record any website
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebEgressRequest {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub audio_only: bool,
    #[prost(bool, tag="3")]
    pub video_only: bool,
    #[prost(bool, tag="12")]
    pub await_start_signal: bool,
    #[prost(message, repeated, tag="9")]
    pub file_outputs: ::prost::alloc::vec::Vec<EncodedFileOutput>,
    #[prost(message, repeated, tag="10")]
    pub stream_outputs: ::prost::alloc::vec::Vec<StreamOutput>,
    #[prost(message, repeated, tag="11")]
    pub segment_outputs: ::prost::alloc::vec::Vec<SegmentedFileOutput>,
    #[prost(message, repeated, tag="13")]
    pub image_outputs: ::prost::alloc::vec::Vec<ImageOutput>,
    /// deprecated (use _output fields)
    #[prost(oneof="web_egress_request::Output", tags="4, 5, 6")]
    pub output: ::core::option::Option<web_egress_request::Output>,
    #[prost(oneof="web_egress_request::Options", tags="7, 8")]
    pub options: ::core::option::Option<web_egress_request::Options>,
}
/// Nested message and enum types in `WebEgressRequest`.
pub mod web_egress_request {
    /// deprecated (use _output fields)
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="4")]
        File(super::EncodedFileOutput),
        #[prost(message, tag="5")]
        Stream(super::StreamOutput),
        #[prost(message, tag="6")]
        Segments(super::SegmentedFileOutput),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        #[prost(enumeration="super::EncodingOptionsPreset", tag="7")]
        Preset(i32),
        #[prost(message, tag="8")]
        Advanced(super::EncodingOptions),
    }
}
/// record audio and video from a single participant
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantEgressRequest {
    /// required
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// required
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    /// (default false)
    #[prost(bool, tag="3")]
    pub screen_share: bool,
    #[prost(message, repeated, tag="6")]
    pub file_outputs: ::prost::alloc::vec::Vec<EncodedFileOutput>,
    #[prost(message, repeated, tag="7")]
    pub stream_outputs: ::prost::alloc::vec::Vec<StreamOutput>,
    #[prost(message, repeated, tag="8")]
    pub segment_outputs: ::prost::alloc::vec::Vec<SegmentedFileOutput>,
    #[prost(message, repeated, tag="9")]
    pub image_outputs: ::prost::alloc::vec::Vec<ImageOutput>,
    #[prost(oneof="participant_egress_request::Options", tags="4, 5")]
    pub options: ::core::option::Option<participant_egress_request::Options>,
}
/// Nested message and enum types in `ParticipantEgressRequest`.
pub mod participant_egress_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// (default H264_720P_30)
        #[prost(enumeration="super::EncodingOptionsPreset", tag="4")]
        Preset(i32),
        /// (optional)
        #[prost(message, tag="5")]
        Advanced(super::EncodingOptions),
    }
}
/// containerize up to one audio and one video track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackCompositeEgressRequest {
    /// required
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// (optional)
    #[prost(string, tag="2")]
    pub audio_track_id: ::prost::alloc::string::String,
    /// (optional)
    #[prost(string, tag="3")]
    pub video_track_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="11")]
    pub file_outputs: ::prost::alloc::vec::Vec<EncodedFileOutput>,
    #[prost(message, repeated, tag="12")]
    pub stream_outputs: ::prost::alloc::vec::Vec<StreamOutput>,
    #[prost(message, repeated, tag="13")]
    pub segment_outputs: ::prost::alloc::vec::Vec<SegmentedFileOutput>,
    #[prost(message, repeated, tag="14")]
    pub image_outputs: ::prost::alloc::vec::Vec<ImageOutput>,
    /// deprecated (use _output fields)
    #[prost(oneof="track_composite_egress_request::Output", tags="4, 5, 8")]
    pub output: ::core::option::Option<track_composite_egress_request::Output>,
    #[prost(oneof="track_composite_egress_request::Options", tags="6, 7")]
    pub options: ::core::option::Option<track_composite_egress_request::Options>,
}
/// Nested message and enum types in `TrackCompositeEgressRequest`.
pub mod track_composite_egress_request {
    /// deprecated (use _output fields)
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="4")]
        File(super::EncodedFileOutput),
        #[prost(message, tag="5")]
        Stream(super::StreamOutput),
        #[prost(message, tag="8")]
        Segments(super::SegmentedFileOutput),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// (default H264_720P_30)
        #[prost(enumeration="super::EncodingOptionsPreset", tag="6")]
        Preset(i32),
        /// (optional)
        #[prost(message, tag="7")]
        Advanced(super::EncodingOptions),
    }
}
/// record tracks individually, without transcoding
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackEgressRequest {
    /// required
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// required
    #[prost(string, tag="2")]
    pub track_id: ::prost::alloc::string::String,
    /// required
    #[prost(oneof="track_egress_request::Output", tags="3, 4")]
    pub output: ::core::option::Option<track_egress_request::Output>,
}
/// Nested message and enum types in `TrackEgressRequest`.
pub mod track_egress_request {
    /// required
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="3")]
        File(super::DirectFileOutput),
        #[prost(string, tag="4")]
        WebsocketUrl(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncodedFileOutput {
    /// (optional)
    #[prost(enumeration="EncodedFileType", tag="1")]
    pub file_type: i32,
    /// see egress docs for templating (default {room_name}-{time})
    #[prost(string, tag="2")]
    pub filepath: ::prost::alloc::string::String,
    /// disable upload of manifest file (default false)
    #[prost(bool, tag="6")]
    pub disable_manifest: bool,
    #[prost(oneof="encoded_file_output::Output", tags="3, 4, 5, 7")]
    pub output: ::core::option::Option<encoded_file_output::Output>,
}
/// Nested message and enum types in `EncodedFileOutput`.
pub mod encoded_file_output {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="3")]
        S3(super::S3Upload),
        #[prost(message, tag="4")]
        Gcp(super::GcpUpload),
        #[prost(message, tag="5")]
        Azure(super::AzureBlobUpload),
        #[prost(message, tag="7")]
        AliOss(super::AliOssUpload),
    }
}
/// Used to generate HLS segments or other kind of segmented output
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentedFileOutput {
    /// (optional)
    #[prost(enumeration="SegmentedFileProtocol", tag="1")]
    pub protocol: i32,
    /// (optional)
    #[prost(string, tag="2")]
    pub filename_prefix: ::prost::alloc::string::String,
    /// (optional)
    #[prost(string, tag="3")]
    pub playlist_name: ::prost::alloc::string::String,
    /// (optional, disabled if not provided). Path of a live playlist
    #[prost(string, tag="11")]
    pub live_playlist_name: ::prost::alloc::string::String,
    /// in seconds (optional)
    #[prost(uint32, tag="4")]
    pub segment_duration: u32,
    /// (optional, default INDEX)
    #[prost(enumeration="SegmentedFileSuffix", tag="10")]
    pub filename_suffix: i32,
    /// disable upload of manifest file (default false)
    #[prost(bool, tag="8")]
    pub disable_manifest: bool,
    /// required
    #[prost(oneof="segmented_file_output::Output", tags="5, 6, 7, 9")]
    pub output: ::core::option::Option<segmented_file_output::Output>,
}
/// Nested message and enum types in `SegmentedFileOutput`.
pub mod segmented_file_output {
    /// required
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="5")]
        S3(super::S3Upload),
        #[prost(message, tag="6")]
        Gcp(super::GcpUpload),
        #[prost(message, tag="7")]
        Azure(super::AzureBlobUpload),
        #[prost(message, tag="9")]
        AliOss(super::AliOssUpload),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectFileOutput {
    /// see egress docs for templating (default {track_id}-{time})
    #[prost(string, tag="1")]
    pub filepath: ::prost::alloc::string::String,
    /// disable upload of manifest file (default false)
    #[prost(bool, tag="5")]
    pub disable_manifest: bool,
    #[prost(oneof="direct_file_output::Output", tags="2, 3, 4, 6")]
    pub output: ::core::option::Option<direct_file_output::Output>,
}
/// Nested message and enum types in `DirectFileOutput`.
pub mod direct_file_output {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="2")]
        S3(super::S3Upload),
        #[prost(message, tag="3")]
        Gcp(super::GcpUpload),
        #[prost(message, tag="4")]
        Azure(super::AzureBlobUpload),
        #[prost(message, tag="6")]
        AliOss(super::AliOssUpload),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageOutput {
    /// in seconds (required)
    #[prost(uint32, tag="1")]
    pub capture_interval: u32,
    /// (optional, defaults to track width)
    #[prost(int32, tag="2")]
    pub width: i32,
    /// (optional, defaults to track height)
    #[prost(int32, tag="3")]
    pub height: i32,
    /// (optional)
    #[prost(string, tag="4")]
    pub filename_prefix: ::prost::alloc::string::String,
    /// (optional, default INDEX)
    #[prost(enumeration="ImageFileSuffix", tag="5")]
    pub filename_suffix: i32,
    /// (optional)
    #[prost(enumeration="ImageCodec", tag="6")]
    pub image_codec: i32,
    /// disable upload of manifest file (default false)
    #[prost(bool, tag="7")]
    pub disable_manifest: bool,
    /// required
    #[prost(oneof="image_output::Output", tags="8, 9, 10, 11")]
    pub output: ::core::option::Option<image_output::Output>,
}
/// Nested message and enum types in `ImageOutput`.
pub mod image_output {
    /// required
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="8")]
        S3(super::S3Upload),
        #[prost(message, tag="9")]
        Gcp(super::GcpUpload),
        #[prost(message, tag="10")]
        Azure(super::AzureBlobUpload),
        #[prost(message, tag="11")]
        AliOss(super::AliOssUpload),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3Upload {
    #[prost(string, tag="1")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub force_path_style: bool,
    #[prost(map="string, string", tag="7")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="8")]
    pub tagging: ::prost::alloc::string::String,
    /// Content-Disposition header
    #[prost(string, tag="9")]
    pub content_disposition: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpUpload {
    /// service account credentials serialized in JSON "credentials.json"
    #[prost(string, tag="1")]
    pub credentials: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bucket: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobUpload {
    #[prost(string, tag="1")]
    pub account_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub container_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliOssUpload {
    #[prost(string, tag="1")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bucket: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOutput {
    /// required
    #[prost(enumeration="StreamProtocol", tag="1")]
    pub protocol: i32,
    /// required
    #[prost(string, repeated, tag="2")]
    pub urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncodingOptions {
    /// (default 1920)
    #[prost(int32, tag="1")]
    pub width: i32,
    /// (default 1080)
    #[prost(int32, tag="2")]
    pub height: i32,
    /// (default 24)
    #[prost(int32, tag="3")]
    pub depth: i32,
    /// (default 30)
    #[prost(int32, tag="4")]
    pub framerate: i32,
    /// (default OPUS)
    #[prost(enumeration="AudioCodec", tag="5")]
    pub audio_codec: i32,
    /// (default 128)
    #[prost(int32, tag="6")]
    pub audio_bitrate: i32,
    /// quality setting on audio encoder
    #[prost(int32, tag="11")]
    pub audio_quality: i32,
    /// (default 44100)
    #[prost(int32, tag="7")]
    pub audio_frequency: i32,
    /// (default H264_MAIN)
    #[prost(enumeration="VideoCodec", tag="8")]
    pub video_codec: i32,
    /// (default 4500)
    #[prost(int32, tag="9")]
    pub video_bitrate: i32,
    /// quality setting on video encoder
    #[prost(int32, tag="12")]
    pub video_quality: i32,
    /// in seconds (default 4s for streaming, segment duration for segmented output, encoder default for files)
    #[prost(double, tag="10")]
    pub key_frame_interval: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLayoutRequest {
    #[prost(string, tag="1")]
    pub egress_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub layout: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStreamRequest {
    #[prost(string, tag="1")]
    pub egress_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub add_output_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub remove_output_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEgressRequest {
    /// (optional, filter by room name)
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// (optional, filter by egress ID)
    #[prost(string, tag="2")]
    pub egress_id: ::prost::alloc::string::String,
    /// (optional, list active egress only)
    #[prost(bool, tag="3")]
    pub active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEgressResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<EgressInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopEgressRequest {
    #[prost(string, tag="1")]
    pub egress_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgressInfo {
    #[prost(string, tag="1")]
    pub egress_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(enumeration="EgressStatus", tag="3")]
    pub status: i32,
    #[prost(int64, tag="10")]
    pub started_at: i64,
    #[prost(int64, tag="11")]
    pub ended_at: i64,
    #[prost(int64, tag="18")]
    pub updated_at: i64,
    #[prost(string, tag="9")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="15")]
    pub stream_results: ::prost::alloc::vec::Vec<StreamInfo>,
    #[prost(message, repeated, tag="16")]
    pub file_results: ::prost::alloc::vec::Vec<FileInfo>,
    #[prost(message, repeated, tag="17")]
    pub segment_results: ::prost::alloc::vec::Vec<SegmentsInfo>,
    #[prost(message, repeated, tag="20")]
    pub image_results: ::prost::alloc::vec::Vec<ImagesInfo>,
    #[prost(oneof="egress_info::Request", tags="4, 14, 19, 5, 6")]
    pub request: ::core::option::Option<egress_info::Request>,
    /// deprecated (use _result fields)
    #[prost(oneof="egress_info::Result", tags="7, 8, 12")]
    pub result: ::core::option::Option<egress_info::Result>,
}
/// Nested message and enum types in `EgressInfo`.
pub mod egress_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="4")]
        RoomComposite(super::RoomCompositeEgressRequest),
        #[prost(message, tag="14")]
        Web(super::WebEgressRequest),
        #[prost(message, tag="19")]
        Participant(super::ParticipantEgressRequest),
        #[prost(message, tag="5")]
        TrackComposite(super::TrackCompositeEgressRequest),
        #[prost(message, tag="6")]
        Track(super::TrackEgressRequest),
    }
    /// deprecated (use _result fields)
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="7")]
        Stream(super::StreamInfoList),
        #[prost(message, tag="8")]
        File(super::FileInfo),
        #[prost(message, tag="12")]
        Segments(super::SegmentsInfo),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfoList {
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<StreamInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub started_at: i64,
    #[prost(int64, tag="3")]
    pub ended_at: i64,
    #[prost(int64, tag="4")]
    pub duration: i64,
    #[prost(enumeration="stream_info::Status", tag="5")]
    pub status: i32,
    #[prost(string, tag="6")]
    pub error: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StreamInfo`.
pub mod stream_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Active = 0,
        Finished = 1,
        Failed = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Active => "ACTIVE",
                Status::Finished => "FINISHED",
                Status::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTIVE" => Some(Self::Active),
                "FINISHED" => Some(Self::Finished),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub started_at: i64,
    #[prost(int64, tag="3")]
    pub ended_at: i64,
    #[prost(int64, tag="6")]
    pub duration: i64,
    #[prost(int64, tag="4")]
    pub size: i64,
    #[prost(string, tag="5")]
    pub location: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentsInfo {
    #[prost(string, tag="1")]
    pub playlist_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub live_playlist_name: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub duration: i64,
    #[prost(int64, tag="3")]
    pub size: i64,
    #[prost(string, tag="4")]
    pub playlist_location: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub live_playlist_location: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub segment_count: i64,
    #[prost(int64, tag="6")]
    pub started_at: i64,
    #[prost(int64, tag="7")]
    pub ended_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagesInfo {
    #[prost(int64, tag="1")]
    pub image_count: i64,
    #[prost(int64, tag="2")]
    pub started_at: i64,
    #[prost(int64, tag="3")]
    pub ended_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoParticipantEgress {
    #[prost(message, repeated, tag="3")]
    pub file_outputs: ::prost::alloc::vec::Vec<EncodedFileOutput>,
    #[prost(message, repeated, tag="4")]
    pub segment_outputs: ::prost::alloc::vec::Vec<SegmentedFileOutput>,
    #[prost(oneof="auto_participant_egress::Options", tags="1, 2")]
    pub options: ::core::option::Option<auto_participant_egress::Options>,
}
/// Nested message and enum types in `AutoParticipantEgress`.
pub mod auto_participant_egress {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// (default H264_720P_30)
        #[prost(enumeration="super::EncodingOptionsPreset", tag="1")]
        Preset(i32),
        /// (optional)
        #[prost(message, tag="2")]
        Advanced(super::EncodingOptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoTrackEgress {
    /// see docs for templating (default {track_id}-{time})
    #[prost(string, tag="1")]
    pub filepath: ::prost::alloc::string::String,
    /// disables upload of json manifest file (default false)
    #[prost(bool, tag="5")]
    pub disable_manifest: bool,
    #[prost(oneof="auto_track_egress::Output", tags="2, 3, 4")]
    pub output: ::core::option::Option<auto_track_egress::Output>,
}
/// Nested message and enum types in `AutoTrackEgress`.
pub mod auto_track_egress {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="2")]
        S3(super::S3Upload),
        #[prost(message, tag="3")]
        Gcp(super::GcpUpload),
        #[prost(message, tag="4")]
        Azure(super::AzureBlobUpload),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodedFileType {
    /// file type chosen based on codecs
    DefaultFiletype = 0,
    Mp4 = 1,
    Ogg = 2,
}
impl EncodedFileType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncodedFileType::DefaultFiletype => "DEFAULT_FILETYPE",
            EncodedFileType::Mp4 => "MP4",
            EncodedFileType::Ogg => "OGG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_FILETYPE" => Some(Self::DefaultFiletype),
            "MP4" => Some(Self::Mp4),
            "OGG" => Some(Self::Ogg),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SegmentedFileProtocol {
    DefaultSegmentedFileProtocol = 0,
    HlsProtocol = 1,
}
impl SegmentedFileProtocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SegmentedFileProtocol::DefaultSegmentedFileProtocol => "DEFAULT_SEGMENTED_FILE_PROTOCOL",
            SegmentedFileProtocol::HlsProtocol => "HLS_PROTOCOL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_SEGMENTED_FILE_PROTOCOL" => Some(Self::DefaultSegmentedFileProtocol),
            "HLS_PROTOCOL" => Some(Self::HlsProtocol),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SegmentedFileSuffix {
    Index = 0,
    Timestamp = 1,
}
impl SegmentedFileSuffix {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SegmentedFileSuffix::Index => "INDEX",
            SegmentedFileSuffix::Timestamp => "TIMESTAMP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INDEX" => Some(Self::Index),
            "TIMESTAMP" => Some(Self::Timestamp),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImageFileSuffix {
    ImageSuffixIndex = 0,
    ImageSuffixTimestamp = 1,
}
impl ImageFileSuffix {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImageFileSuffix::ImageSuffixIndex => "IMAGE_SUFFIX_INDEX",
            ImageFileSuffix::ImageSuffixTimestamp => "IMAGE_SUFFIX_TIMESTAMP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMAGE_SUFFIX_INDEX" => Some(Self::ImageSuffixIndex),
            "IMAGE_SUFFIX_TIMESTAMP" => Some(Self::ImageSuffixTimestamp),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StreamProtocol {
    /// protocol chosen based on urls
    DefaultProtocol = 0,
    Rtmp = 1,
}
impl StreamProtocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StreamProtocol::DefaultProtocol => "DEFAULT_PROTOCOL",
            StreamProtocol::Rtmp => "RTMP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT_PROTOCOL" => Some(Self::DefaultProtocol),
            "RTMP" => Some(Self::Rtmp),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodingOptionsPreset {
    ///   1280x720, 30fps, 3000kpbs, H.264_MAIN / OPUS
    H264720p30 = 0,
    ///   1280x720, 60fps, 4500kbps, H.264_MAIN / OPUS
    H264720p60 = 1,
    /// 1920x1080, 30fps, 4500kbps, H.264_MAIN / OPUS
    H2641080p30 = 2,
    /// 1920x1080, 60fps, 6000kbps, H.264_MAIN / OPUS
    H2641080p60 = 3,
    ///   720x1280, 30fps, 3000kpbs, H.264_MAIN / OPUS
    PortraitH264720p30 = 4,
    ///   720x1280, 60fps, 4500kbps, H.264_MAIN / OPUS
    PortraitH264720p60 = 5,
    /// 1080x1920, 30fps, 4500kbps, H.264_MAIN / OPUS
    PortraitH2641080p30 = 6,
    /// 1080x1920, 60fps, 6000kbps, H.264_MAIN / OPUS
    PortraitH2641080p60 = 7,
}
impl EncodingOptionsPreset {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncodingOptionsPreset::H264720p30 => "H264_720P_30",
            EncodingOptionsPreset::H264720p60 => "H264_720P_60",
            EncodingOptionsPreset::H2641080p30 => "H264_1080P_30",
            EncodingOptionsPreset::H2641080p60 => "H264_1080P_60",
            EncodingOptionsPreset::PortraitH264720p30 => "PORTRAIT_H264_720P_30",
            EncodingOptionsPreset::PortraitH264720p60 => "PORTRAIT_H264_720P_60",
            EncodingOptionsPreset::PortraitH2641080p30 => "PORTRAIT_H264_1080P_30",
            EncodingOptionsPreset::PortraitH2641080p60 => "PORTRAIT_H264_1080P_60",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "H264_720P_30" => Some(Self::H264720p30),
            "H264_720P_60" => Some(Self::H264720p60),
            "H264_1080P_30" => Some(Self::H2641080p30),
            "H264_1080P_60" => Some(Self::H2641080p60),
            "PORTRAIT_H264_720P_30" => Some(Self::PortraitH264720p30),
            "PORTRAIT_H264_720P_60" => Some(Self::PortraitH264720p60),
            "PORTRAIT_H264_1080P_30" => Some(Self::PortraitH2641080p30),
            "PORTRAIT_H264_1080P_60" => Some(Self::PortraitH2641080p60),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EgressStatus {
    EgressStarting = 0,
    EgressActive = 1,
    EgressEnding = 2,
    EgressComplete = 3,
    EgressFailed = 4,
    EgressAborted = 5,
    EgressLimitReached = 6,
}
impl EgressStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EgressStatus::EgressStarting => "EGRESS_STARTING",
            EgressStatus::EgressActive => "EGRESS_ACTIVE",
            EgressStatus::EgressEnding => "EGRESS_ENDING",
            EgressStatus::EgressComplete => "EGRESS_COMPLETE",
            EgressStatus::EgressFailed => "EGRESS_FAILED",
            EgressStatus::EgressAborted => "EGRESS_ABORTED",
            EgressStatus::EgressLimitReached => "EGRESS_LIMIT_REACHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EGRESS_STARTING" => Some(Self::EgressStarting),
            "EGRESS_ACTIVE" => Some(Self::EgressActive),
            "EGRESS_ENDING" => Some(Self::EgressEnding),
            "EGRESS_COMPLETE" => Some(Self::EgressComplete),
            "EGRESS_FAILED" => Some(Self::EgressFailed),
            "EGRESS_ABORTED" => Some(Self::EgressAborted),
            "EGRESS_LIMIT_REACHED" => Some(Self::EgressLimitReached),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalRequest {
    #[prost(oneof="signal_request::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16")]
    pub message: ::core::option::Option<signal_request::Message>,
}
/// Nested message and enum types in `SignalRequest`.
pub mod signal_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// initial join exchange, for publisher
        #[prost(message, tag="1")]
        Offer(super::SessionDescription),
        /// participant answering publisher offer
        #[prost(message, tag="2")]
        Answer(super::SessionDescription),
        #[prost(message, tag="3")]
        Trickle(super::TrickleRequest),
        #[prost(message, tag="4")]
        AddTrack(super::AddTrackRequest),
        /// mute the participant's published tracks
        #[prost(message, tag="5")]
        Mute(super::MuteTrackRequest),
        /// Subscribe or unsubscribe from tracks
        #[prost(message, tag="6")]
        Subscription(super::UpdateSubscription),
        /// Update settings of subscribed tracks
        #[prost(message, tag="7")]
        TrackSetting(super::UpdateTrackSettings),
        /// Immediately terminate session
        #[prost(message, tag="8")]
        Leave(super::LeaveRequest),
        /// Update published video layers
        #[prost(message, tag="10")]
        UpdateLayers(super::UpdateVideoLayers),
        /// Update subscriber permissions
        #[prost(message, tag="11")]
        SubscriptionPermission(super::SubscriptionPermission),
        /// sync client's subscribe state to server during reconnect
        #[prost(message, tag="12")]
        SyncState(super::SyncState),
        /// Simulate conditions, for client validations
        #[prost(message, tag="13")]
        Simulate(super::SimulateScenario),
        /// client triggered ping to server
        ///
        /// deprecated by ping_req (message Ping)
        #[prost(int64, tag="14")]
        Ping(i64),
        /// update a participant's own metadata and/or name
        #[prost(message, tag="15")]
        UpdateMetadata(super::UpdateParticipantMetadata),
        #[prost(message, tag="16")]
        PingReq(super::Ping),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalResponse {
    #[prost(oneof="signal_response::Message", tags="1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21")]
    pub message: ::core::option::Option<signal_response::Message>,
}
/// Nested message and enum types in `SignalResponse`.
pub mod signal_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// sent when join is accepted
        #[prost(message, tag="1")]
        Join(super::JoinResponse),
        /// sent when server answers publisher
        #[prost(message, tag="2")]
        Answer(super::SessionDescription),
        /// sent when server is sending subscriber an offer
        #[prost(message, tag="3")]
        Offer(super::SessionDescription),
        /// sent when an ICE candidate is available
        #[prost(message, tag="4")]
        Trickle(super::TrickleRequest),
        /// sent when participants in the room has changed
        #[prost(message, tag="5")]
        Update(super::ParticipantUpdate),
        /// sent to the participant when their track has been published
        #[prost(message, tag="6")]
        TrackPublished(super::TrackPublishedResponse),
        /// Immediately terminate session
        #[prost(message, tag="8")]
        Leave(super::LeaveRequest),
        /// server initiated mute
        #[prost(message, tag="9")]
        Mute(super::MuteTrackRequest),
        /// indicates changes to speaker status, including when they've gone to not speaking
        #[prost(message, tag="10")]
        SpeakersChanged(super::SpeakersChanged),
        /// sent when metadata of the room has changed
        #[prost(message, tag="11")]
        RoomUpdate(super::RoomUpdate),
        /// when connection quality changed
        #[prost(message, tag="12")]
        ConnectionQuality(super::ConnectionQualityUpdate),
        /// when streamed tracks state changed, used to notify when any of the streams were paused due to
        /// congestion
        #[prost(message, tag="13")]
        StreamStateUpdate(super::StreamStateUpdate),
        /// when max subscribe quality changed, used by dynamic broadcasting to disable unused layers
        #[prost(message, tag="14")]
        SubscribedQualityUpdate(super::SubscribedQualityUpdate),
        /// when subscription permission changed
        #[prost(message, tag="15")]
        SubscriptionPermissionUpdate(super::SubscriptionPermissionUpdate),
        /// update the token the client was using, to prevent an active client from using an expired token
        #[prost(string, tag="16")]
        RefreshToken(::prost::alloc::string::String),
        /// server initiated track unpublish
        #[prost(message, tag="17")]
        TrackUnpublished(super::TrackUnpublishedResponse),
        /// respond to ping
        ///
        /// deprecated by pong_resp (message Pong)
        #[prost(int64, tag="18")]
        Pong(i64),
        /// sent when client reconnects
        #[prost(message, tag="19")]
        Reconnect(super::ReconnectResponse),
        /// respond to Ping
        #[prost(message, tag="20")]
        PongResp(super::Pong),
        /// Subscription response, client should not expect any media from this subscription if it fails
        #[prost(message, tag="21")]
        SubscriptionResponse(super::SubscriptionResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulcastCodec {
    #[prost(string, tag="1")]
    pub codec: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTrackRequest {
    /// client ID of track, to match it when RTC track is received
    #[prost(string, tag="1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="TrackType", tag="3")]
    pub r#type: i32,
    /// to be deprecated in favor of layers
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
    /// true to add track and initialize to muted
    #[prost(bool, tag="6")]
    pub muted: bool,
    /// true if DTX (Discontinuous Transmission) is disabled for audio
    #[prost(bool, tag="7")]
    pub disable_dtx: bool,
    #[prost(enumeration="TrackSource", tag="8")]
    pub source: i32,
    #[prost(message, repeated, tag="9")]
    pub layers: ::prost::alloc::vec::Vec<VideoLayer>,
    #[prost(message, repeated, tag="10")]
    pub simulcast_codecs: ::prost::alloc::vec::Vec<SimulcastCodec>,
    /// server ID of track, publish new codec to exist track
    #[prost(string, tag="11")]
    pub sid: ::prost::alloc::string::String,
    #[prost(bool, tag="12")]
    pub stereo: bool,
    /// true if RED (Redundant Encoding) is disabled for audio
    #[prost(bool, tag="13")]
    pub disable_red: bool,
    #[prost(enumeration="encryption::Type", tag="14")]
    pub encryption: i32,
    /// which stream the track belongs to, used to group tracks together.
    /// if not specified, server will infer it from track source to bundle camera/microphone, screenshare/audio together
    #[prost(string, tag="15")]
    pub stream: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrickleRequest {
    #[prost(string, tag="1")]
    pub candidate_init: ::prost::alloc::string::String,
    #[prost(enumeration="SignalTarget", tag="2")]
    pub target: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuteTrackRequest {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub muted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinResponse {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
    #[prost(message, optional, tag="2")]
    pub participant: ::core::option::Option<ParticipantInfo>,
    #[prost(message, repeated, tag="3")]
    pub other_participants: ::prost::alloc::vec::Vec<ParticipantInfo>,
    /// deprecated. use server_info.version instead.
    #[prost(string, tag="4")]
    pub server_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub ice_servers: ::prost::alloc::vec::Vec<IceServer>,
    /// use subscriber as the primary PeerConnection
    #[prost(bool, tag="6")]
    pub subscriber_primary: bool,
    /// when the current server isn't available, return alternate url to retry connection
    /// when this is set, the other fields will be largely empty
    #[prost(string, tag="7")]
    pub alternative_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub client_configuration: ::core::option::Option<ClientConfiguration>,
    /// deprecated. use server_info.region instead.
    #[prost(string, tag="9")]
    pub server_region: ::prost::alloc::string::String,
    #[prost(int32, tag="10")]
    pub ping_timeout: i32,
    #[prost(int32, tag="11")]
    pub ping_interval: i32,
    #[prost(message, optional, tag="12")]
    pub server_info: ::core::option::Option<ServerInfo>,
    /// Server-Injected-Frame byte trailer, used to identify unencrypted frames when e2ee is enabled
    #[prost(bytes="vec", tag="13")]
    pub sif_trailer: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectResponse {
    #[prost(message, repeated, tag="1")]
    pub ice_servers: ::prost::alloc::vec::Vec<IceServer>,
    #[prost(message, optional, tag="2")]
    pub client_configuration: ::core::option::Option<ClientConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPublishedResponse {
    #[prost(string, tag="1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub track: ::core::option::Option<TrackInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUnpublishedResponse {
    #[prost(string, tag="1")]
    pub track_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionDescription {
    /// "answer" | "offer" | "pranswer" | "rollback"
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sdp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantUpdate {
    #[prost(message, repeated, tag="1")]
    pub participants: ::prost::alloc::vec::Vec<ParticipantInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscription {
    #[prost(string, repeated, tag="1")]
    pub track_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="2")]
    pub subscribe: bool,
    #[prost(message, repeated, tag="3")]
    pub participant_tracks: ::prost::alloc::vec::Vec<ParticipantTracks>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackSettings {
    #[prost(string, repeated, tag="1")]
    pub track_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// when true, the track is placed in a paused state, with no new data returned
    #[prost(bool, tag="3")]
    pub disabled: bool,
    /// deprecated in favor of width & height
    #[prost(enumeration="VideoQuality", tag="4")]
    pub quality: i32,
    /// for video, width to receive
    #[prost(uint32, tag="5")]
    pub width: u32,
    /// for video, height to receive
    #[prost(uint32, tag="6")]
    pub height: u32,
    #[prost(uint32, tag="7")]
    pub fps: u32,
    /// subscription priority. 1 being the highest (0 is unset)
    /// when unset, server sill assign priority based on the order of subscription
    /// server will use priority in the following ways:
    /// 1. when subscribed tracks exceed per-participant subscription limit, server will
    ///     pause the lowest priority tracks
    /// 2. when the network is congested, server will assign available bandwidth to
    ///     higher priority tracks first. lowest priority tracks can be paused
    #[prost(uint32, tag="8")]
    pub priority: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveRequest {
    /// sent when server initiates the disconnect due to server-restart
    /// indicates clients should attempt full-reconnect sequence
    #[prost(bool, tag="1")]
    pub can_reconnect: bool,
    #[prost(enumeration="DisconnectReason", tag="2")]
    pub reason: i32,
}
/// message to indicate published video track dimensions are changing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVideoLayers {
    #[prost(string, tag="1")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub layers: ::prost::alloc::vec::Vec<VideoLayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParticipantMetadata {
    #[prost(string, tag="1")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IceServer {
    #[prost(string, repeated, tag="1")]
    pub urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub credential: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeakersChanged {
    #[prost(message, repeated, tag="1")]
    pub speakers: ::prost::alloc::vec::Vec<SpeakerInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomUpdate {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionQualityInfo {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(enumeration="ConnectionQuality", tag="2")]
    pub quality: i32,
    #[prost(float, tag="3")]
    pub score: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionQualityUpdate {
    #[prost(message, repeated, tag="1")]
    pub updates: ::prost::alloc::vec::Vec<ConnectionQualityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStateInfo {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(enumeration="StreamState", tag="3")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStateUpdate {
    #[prost(message, repeated, tag="1")]
    pub stream_states: ::prost::alloc::vec::Vec<StreamStateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribedQuality {
    #[prost(enumeration="VideoQuality", tag="1")]
    pub quality: i32,
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribedCodec {
    #[prost(string, tag="1")]
    pub codec: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub qualities: ::prost::alloc::vec::Vec<SubscribedQuality>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribedQualityUpdate {
    #[prost(string, tag="1")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub subscribed_qualities: ::prost::alloc::vec::Vec<SubscribedQuality>,
    #[prost(message, repeated, tag="3")]
    pub subscribed_codecs: ::prost::alloc::vec::Vec<SubscribedCodec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPermission {
    /// permission could be granted either by participant sid or identity
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub all_tracks: bool,
    #[prost(string, repeated, tag="3")]
    pub track_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub participant_identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionPermission {
    #[prost(bool, tag="1")]
    pub all_participants: bool,
    #[prost(message, repeated, tag="2")]
    pub track_permissions: ::prost::alloc::vec::Vec<TrackPermission>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionPermissionUpdate {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub allowed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncState {
    /// last subscribe answer before reconnecting
    #[prost(message, optional, tag="1")]
    pub answer: ::core::option::Option<SessionDescription>,
    #[prost(message, optional, tag="2")]
    pub subscription: ::core::option::Option<UpdateSubscription>,
    #[prost(message, repeated, tag="3")]
    pub publish_tracks: ::prost::alloc::vec::Vec<TrackPublishedResponse>,
    #[prost(message, repeated, tag="4")]
    pub data_channels: ::prost::alloc::vec::Vec<DataChannelInfo>,
    /// last received server side offer before reconnecting
    #[prost(message, optional, tag="5")]
    pub offer: ::core::option::Option<SessionDescription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataChannelInfo {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub id: u32,
    #[prost(enumeration="SignalTarget", tag="3")]
    pub target: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateScenario {
    #[prost(oneof="simulate_scenario::Scenario", tags="1, 2, 3, 4, 5, 6")]
    pub scenario: ::core::option::Option<simulate_scenario::Scenario>,
}
/// Nested message and enum types in `SimulateScenario`.
pub mod simulate_scenario {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scenario {
        /// simulate N seconds of speaker activity
        #[prost(int32, tag="1")]
        SpeakerUpdate(i32),
        /// simulate local node failure
        #[prost(bool, tag="2")]
        NodeFailure(bool),
        /// simulate migration
        #[prost(bool, tag="3")]
        Migration(bool),
        /// server to send leave
        #[prost(bool, tag="4")]
        ServerLeave(bool),
        /// switch candidate protocol to tcp
        #[prost(enumeration="super::CandidateProtocol", tag="5")]
        SwitchCandidateProtocol(i32),
        /// maximum bandwidth for subscribers, in bps
        /// when zero, clears artificial bandwidth limit
        #[prost(int64, tag="6")]
        SubscriberBandwidth(i64),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    /// rtt in milliseconds calculated by client
    #[prost(int64, tag="2")]
    pub rtt: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {
    /// timestamp field of last received ping request
    #[prost(int64, tag="1")]
    pub last_ping_timestamp: i64,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSettings {
    #[prost(message, repeated, tag="1")]
    pub regions: ::prost::alloc::vec::Vec<RegionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfo {
    #[prost(string, tag="1")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub distance: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionResponse {
    #[prost(string, tag="1")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(enumeration="SubscriptionError", tag="2")]
    pub err: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignalTarget {
    Publisher = 0,
    Subscriber = 1,
}
impl SignalTarget {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignalTarget::Publisher => "PUBLISHER",
            SignalTarget::Subscriber => "SUBSCRIBER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PUBLISHER" => Some(Self::Publisher),
            "SUBSCRIBER" => Some(Self::Subscriber),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StreamState {
    Active = 0,
    Paused = 1,
}
impl StreamState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StreamState::Active => "ACTIVE",
            StreamState::Paused => "PAUSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTIVE" => Some(Self::Active),
            "PAUSED" => Some(Self::Paused),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandidateProtocol {
    Udp = 0,
    Tcp = 1,
    Tls = 2,
}
impl CandidateProtocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandidateProtocol::Udp => "UDP",
            CandidateProtocol::Tcp => "TCP",
            CandidateProtocol::Tls => "TLS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UDP" => Some(Self::Udp),
            "TCP" => Some(Self::Tcp),
            "TLS" => Some(Self::Tls),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomRequest {
    /// name of the room
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// number of seconds to keep the room open if no one joins
    #[prost(uint32, tag="2")]
    pub empty_timeout: u32,
    /// limit number of participants that can be in a room
    #[prost(uint32, tag="3")]
    pub max_participants: u32,
    /// override the node room is allocated to, for debugging
    #[prost(string, tag="4")]
    pub node_id: ::prost::alloc::string::String,
    /// metadata of room
    #[prost(string, tag="5")]
    pub metadata: ::prost::alloc::string::String,
    /// egress
    #[prost(message, optional, tag="6")]
    pub egress: ::core::option::Option<RoomEgress>,
    /// playout delay of subscriber
    #[prost(uint32, tag="7")]
    pub min_playout_delay: u32,
    #[prost(uint32, tag="8")]
    pub max_playout_delay: u32,
    /// improves A/V sync when playout_delay set to a value larger than 200ms. It will disables transceiver re-use 
    /// so not recommended for rooms with frequent subscription changes
    #[prost(bool, tag="9")]
    pub sync_streams: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomEgress {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<RoomCompositeEgressRequest>,
    #[prost(message, optional, tag="3")]
    pub participant: ::core::option::Option<AutoParticipantEgress>,
    #[prost(message, optional, tag="2")]
    pub tracks: ::core::option::Option<AutoTrackEgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsRequest {
    /// when set, will only return rooms with name match
    #[prost(string, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsResponse {
    #[prost(message, repeated, tag="1")]
    pub rooms: ::prost::alloc::vec::Vec<Room>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomRequest {
    /// name of the room
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsRequest {
    /// name of the room
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsResponse {
    #[prost(message, repeated, tag="1")]
    pub participants: ::prost::alloc::vec::Vec<ParticipantInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomParticipantIdentity {
    /// name of the room
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    /// identity of the participant
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveParticipantResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuteRoomTrackRequest {
    /// name of the room
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    /// sid of the track to mute
    #[prost(string, tag="3")]
    pub track_sid: ::prost::alloc::string::String,
    /// set to true to mute, false to unmute
    #[prost(bool, tag="4")]
    pub muted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuteRoomTrackResponse {
    #[prost(message, optional, tag="1")]
    pub track: ::core::option::Option<TrackInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParticipantRequest {
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    /// metadata to update. skipping updates if left empty
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// set to update the participant's permissions
    #[prost(message, optional, tag="4")]
    pub permission: ::core::option::Option<ParticipantPermission>,
    /// display name to update
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionsRequest {
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
    /// list of sids of tracks
    #[prost(string, repeated, tag="3")]
    pub track_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// set to true to subscribe, false to unsubscribe from tracks
    #[prost(bool, tag="4")]
    pub subscribe: bool,
    /// list of participants and their tracks
    #[prost(message, repeated, tag="5")]
    pub participant_tracks: ::prost::alloc::vec::Vec<ParticipantTracks>,
}
/// empty for now
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendDataRequest {
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="data_packet::Kind", tag="3")]
    pub kind: i32,
    /// mark deprecated
    #[deprecated]
    #[prost(string, repeated, tag="4")]
    pub destination_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// when set, only forward to these identities
    #[prost(string, repeated, tag="6")]
    pub destination_identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendDataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRoomMetadataRequest {
    #[prost(string, tag="1")]
    pub room: ::prost::alloc::string::String,
    /// metadata to update. skipping updates if left empty
    #[prost(string, tag="2")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngressRequest {
    #[prost(enumeration="IngressInput", tag="1")]
    pub input_type: i32,
    /// Where to pull media from, only for URL input type
    #[prost(string, tag="9")]
    pub url: ::prost::alloc::string::String,
    /// User provided identifier for the ingress
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// room to publish to
    #[prost(string, tag="3")]
    pub room_name: ::prost::alloc::string::String,
    /// publish as participant
    #[prost(string, tag="4")]
    pub participant_identity: ::prost::alloc::string::String,
    /// name of publishing participant (used for display only)
    #[prost(string, tag="5")]
    pub participant_name: ::prost::alloc::string::String,
    /// whether to pass through the incoming media without transcoding, only compatible with some input types
    #[prost(bool, tag="8")]
    pub bypass_transcoding: bool,
    #[prost(message, optional, tag="6")]
    pub audio: ::core::option::Option<IngressAudioOptions>,
    #[prost(message, optional, tag="7")]
    pub video: ::core::option::Option<IngressVideoOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressAudioOptions {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="TrackSource", tag="2")]
    pub source: i32,
    #[prost(oneof="ingress_audio_options::EncodingOptions", tags="3, 4")]
    pub encoding_options: ::core::option::Option<ingress_audio_options::EncodingOptions>,
}
/// Nested message and enum types in `IngressAudioOptions`.
pub mod ingress_audio_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EncodingOptions {
        #[prost(enumeration="super::IngressAudioEncodingPreset", tag="3")]
        Preset(i32),
        #[prost(message, tag="4")]
        Options(super::IngressAudioEncodingOptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressVideoOptions {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="TrackSource", tag="2")]
    pub source: i32,
    #[prost(oneof="ingress_video_options::EncodingOptions", tags="3, 4")]
    pub encoding_options: ::core::option::Option<ingress_video_options::EncodingOptions>,
}
/// Nested message and enum types in `IngressVideoOptions`.
pub mod ingress_video_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EncodingOptions {
        #[prost(enumeration="super::IngressVideoEncodingPreset", tag="3")]
        Preset(i32),
        #[prost(message, tag="4")]
        Options(super::IngressVideoEncodingOptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressAudioEncodingOptions {
    /// desired audio codec to publish to room
    #[prost(enumeration="AudioCodec", tag="1")]
    pub audio_codec: i32,
    #[prost(uint32, tag="2")]
    pub bitrate: u32,
    #[prost(bool, tag="3")]
    pub disable_dtx: bool,
    #[prost(uint32, tag="4")]
    pub channels: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressVideoEncodingOptions {
    /// desired codec to publish to room
    #[prost(enumeration="VideoCodec", tag="1")]
    pub video_codec: i32,
    #[prost(double, tag="2")]
    pub frame_rate: f64,
    /// simulcast layers to publish, when empty, should usually be set to layers at 1/2 and 1/4 of the dimensions
    #[prost(message, repeated, tag="3")]
    pub layers: ::prost::alloc::vec::Vec<VideoLayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressInfo {
    #[prost(string, tag="1")]
    pub ingress_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub stream_key: ::prost::alloc::string::String,
    /// URL to point the encoder to for push (RTMP, WHIP), or location to pull media from for pull (URL)
    #[prost(string, tag="4")]
    pub url: ::prost::alloc::string::String,
    /// for RTMP input, it'll be a rtmp:// URL
    /// for FILE input, it'll be a http:// URL
    /// for SRT input, it'll be a srt:// URL
    #[prost(enumeration="IngressInput", tag="5")]
    pub input_type: i32,
    #[prost(bool, tag="13")]
    pub bypass_transcoding: bool,
    #[prost(message, optional, tag="6")]
    pub audio: ::core::option::Option<IngressAudioOptions>,
    #[prost(message, optional, tag="7")]
    pub video: ::core::option::Option<IngressVideoOptions>,
    #[prost(string, tag="8")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub participant_name: ::prost::alloc::string::String,
    #[prost(bool, tag="11")]
    pub reusable: bool,
    /// Description of error/stream non compliance and debug info for publisher otherwise (received bitrate, resolution, bandwidth)
    #[prost(message, optional, tag="12")]
    pub state: ::core::option::Option<IngressState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressState {
    #[prost(enumeration="ingress_state::Status", tag="1")]
    pub status: i32,
    /// Error/non compliance description if any
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub video: ::core::option::Option<InputVideoState>,
    #[prost(message, optional, tag="4")]
    pub audio: ::core::option::Option<InputAudioState>,
    /// ID of the current/previous room published to
    #[prost(string, tag="5")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub started_at: i64,
    #[prost(int64, tag="8")]
    pub ended_at: i64,
    #[prost(string, tag="9")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub tracks: ::prost::alloc::vec::Vec<TrackInfo>,
}
/// Nested message and enum types in `IngressState`.
pub mod ingress_state {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        EndpointInactive = 0,
        EndpointBuffering = 1,
        EndpointPublishing = 2,
        EndpointError = 3,
        EndpointComplete = 4,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::EndpointInactive => "ENDPOINT_INACTIVE",
                Status::EndpointBuffering => "ENDPOINT_BUFFERING",
                Status::EndpointPublishing => "ENDPOINT_PUBLISHING",
                Status::EndpointError => "ENDPOINT_ERROR",
                Status::EndpointComplete => "ENDPOINT_COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENDPOINT_INACTIVE" => Some(Self::EndpointInactive),
                "ENDPOINT_BUFFERING" => Some(Self::EndpointBuffering),
                "ENDPOINT_PUBLISHING" => Some(Self::EndpointPublishing),
                "ENDPOINT_ERROR" => Some(Self::EndpointError),
                "ENDPOINT_COMPLETE" => Some(Self::EndpointComplete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputVideoState {
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub average_bitrate: u32,
    #[prost(uint32, tag="3")]
    pub width: u32,
    #[prost(uint32, tag="4")]
    pub height: u32,
    #[prost(double, tag="5")]
    pub framerate: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAudioState {
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub average_bitrate: u32,
    #[prost(uint32, tag="3")]
    pub channels: u32,
    #[prost(uint32, tag="4")]
    pub sample_rate: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIngressRequest {
    #[prost(string, tag="1")]
    pub ingress_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub participant_name: ::prost::alloc::string::String,
    #[prost(bool, optional, tag="8")]
    pub bypass_transcoding: ::core::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub audio: ::core::option::Option<IngressAudioOptions>,
    #[prost(message, optional, tag="7")]
    pub video: ::core::option::Option<IngressVideoOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressRequest {
    /// when blank, lists all ingress endpoints
    ///
    /// (optional, filter by room name)
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    /// (optional, filter by ingress ID)
    #[prost(string, tag="2")]
    pub ingress_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<IngressInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIngressRequest {
    #[prost(string, tag="1")]
    pub ingress_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngressInput {
    RtmpInput = 0,
    WhipInput = 1,
    /// Pull from the provided URL. Only HTTP url are supported, serving either a single media file or a HLS stream
    UrlInput = 2,
}
impl IngressInput {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IngressInput::RtmpInput => "RTMP_INPUT",
            IngressInput::WhipInput => "WHIP_INPUT",
            IngressInput::UrlInput => "URL_INPUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RTMP_INPUT" => Some(Self::RtmpInput),
            "WHIP_INPUT" => Some(Self::WhipInput),
            "URL_INPUT" => Some(Self::UrlInput),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngressAudioEncodingPreset {
    /// OPUS, 2 channels, 96kbps
    OpusStereo96kbps = 0,
    /// OPUS, 1 channel, 64kbps
    OpusMono64kbs = 1,
}
impl IngressAudioEncodingPreset {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IngressAudioEncodingPreset::OpusStereo96kbps => "OPUS_STEREO_96KBPS",
            IngressAudioEncodingPreset::OpusMono64kbs => "OPUS_MONO_64KBS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPUS_STEREO_96KBPS" => Some(Self::OpusStereo96kbps),
            "OPUS_MONO_64KBS" => Some(Self::OpusMono64kbs),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngressVideoEncodingPreset {
    /// 1280x720,  30fps, 1900kbps main layer, 3 layers total
    H264720p30fps3Layers = 0,
    /// 1980x1080, 30fps, 3500kbps main layer, 3 layers total
    H2641080p30fps3Layers = 1,
    ///   960x540,  25fps, 1000kbps  main layer, 2 layers total
    H264540p25fps2Layers = 2,
    /// 1280x720,  30fps, 1900kbps, no simulcast
    H264720p30fps1Layer = 3,
    /// 1980x1080, 30fps, 3500kbps, no simulcast
    H2641080p30fps1Layer = 4,
    /// 1280x720,  30fps, 2500kbps main layer, 3 layers total, higher bitrate for high motion, harder to encode content
    H264720p30fps3LayersHighMotion = 5,
    /// 1980x1080, 30fps, 4500kbps main layer, 3 layers total, higher bitrate for high motion, harder to encode content
    H2641080p30fps3LayersHighMotion = 6,
    ///   960x540,  25fps, 1300kbps  main layer, 2 layers total, higher bitrate for high motion, harder to encode content
    H264540p25fps2LayersHighMotion = 7,
    /// 1280x720,  30fps, 2500kbps, no simulcast, higher bitrate for high motion, harder to encode content
    H264720p30fps1LayerHighMotion = 8,
    /// 1980x1080, 30fps, 4500kbps, no simulcast, higher bitrate for high motion, harder to encode content
    H2641080p30fps1LayerHighMotion = 9,
}
impl IngressVideoEncodingPreset {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IngressVideoEncodingPreset::H264720p30fps3Layers => "H264_720P_30FPS_3_LAYERS",
            IngressVideoEncodingPreset::H2641080p30fps3Layers => "H264_1080P_30FPS_3_LAYERS",
            IngressVideoEncodingPreset::H264540p25fps2Layers => "H264_540P_25FPS_2_LAYERS",
            IngressVideoEncodingPreset::H264720p30fps1Layer => "H264_720P_30FPS_1_LAYER",
            IngressVideoEncodingPreset::H2641080p30fps1Layer => "H264_1080P_30FPS_1_LAYER",
            IngressVideoEncodingPreset::H264720p30fps3LayersHighMotion => "H264_720P_30FPS_3_LAYERS_HIGH_MOTION",
            IngressVideoEncodingPreset::H2641080p30fps3LayersHighMotion => "H264_1080P_30FPS_3_LAYERS_HIGH_MOTION",
            IngressVideoEncodingPreset::H264540p25fps2LayersHighMotion => "H264_540P_25FPS_2_LAYERS_HIGH_MOTION",
            IngressVideoEncodingPreset::H264720p30fps1LayerHighMotion => "H264_720P_30FPS_1_LAYER_HIGH_MOTION",
            IngressVideoEncodingPreset::H2641080p30fps1LayerHighMotion => "H264_1080P_30FPS_1_LAYER_HIGH_MOTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "H264_720P_30FPS_3_LAYERS" => Some(Self::H264720p30fps3Layers),
            "H264_1080P_30FPS_3_LAYERS" => Some(Self::H2641080p30fps3Layers),
            "H264_540P_25FPS_2_LAYERS" => Some(Self::H264540p25fps2Layers),
            "H264_720P_30FPS_1_LAYER" => Some(Self::H264720p30fps1Layer),
            "H264_1080P_30FPS_1_LAYER" => Some(Self::H2641080p30fps1Layer),
            "H264_720P_30FPS_3_LAYERS_HIGH_MOTION" => Some(Self::H264720p30fps3LayersHighMotion),
            "H264_1080P_30FPS_3_LAYERS_HIGH_MOTION" => Some(Self::H2641080p30fps3LayersHighMotion),
            "H264_540P_25FPS_2_LAYERS_HIGH_MOTION" => Some(Self::H264540p25fps2LayersHighMotion),
            "H264_720P_30FPS_1_LAYER_HIGH_MOTION" => Some(Self::H264720p30fps1LayerHighMotion),
            "H264_1080P_30FPS_1_LAYER_HIGH_MOTION" => Some(Self::H2641080p30fps1LayerHighMotion),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookEvent {
    /// one of room_started, room_finished, participant_joined, participant_left,
    /// track_published, track_unpublished, egress_started, egress_updated, egress_ended,
    /// ingress_started, ingress_ended
    #[prost(string, tag="1")]
    pub event: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub room: ::core::option::Option<Room>,
    /// set when event is participant_* or track_*
    #[prost(message, optional, tag="3")]
    pub participant: ::core::option::Option<ParticipantInfo>,
    /// set when event is egress_*
    #[prost(message, optional, tag="9")]
    pub egress_info: ::core::option::Option<EgressInfo>,
    /// set when event is ingress_*
    #[prost(message, optional, tag="10")]
    pub ingress_info: ::core::option::Option<IngressInfo>,
    /// set when event is track_*
    #[prost(message, optional, tag="8")]
    pub track: ::core::option::Option<TrackInfo>,
    /// unique event uuid
    #[prost(string, tag="6")]
    pub id: ::prost::alloc::string::String,
    /// timestamp in seconds
    #[prost(int64, tag="7")]
    pub created_at: i64,
    #[prost(int32, tag="11")]
    pub num_dropped: i32,
}
include!("livekit.serde.rs");
// @@protoc_insertion_point(module)
