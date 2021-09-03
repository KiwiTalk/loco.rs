use serde::{Deserialize, Serialize};

/// Checkin response.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Checkin {
    /// IPv4 loco host.
    pub host: String,

    /// IPv6 loco host.
    #[serde(rename = "host6")]
    pub host_ipv6: String,

    /// Loco port.
    pub port: i32,

    /// Cache expire time.
    #[serde(rename = "cacheExpire")]
    pub cache_expire: i32,

    /// IPv4 call server host.
    #[serde(rename = "cshost")]
    pub call_server_host: String,

    /// Call server port.
    #[serde(rename = "csport")]
    pub call_server_port: i32,

    /// IPv6 call server host.
    #[serde(rename = "cshost6")]
    pub call_server_host_ipv6: String,

    /// IPv4 video server host.
    #[serde(rename = "vsshost")]
    pub video_server_host: String,

    /// Video server port.
    #[serde(rename = "vssport")]
    pub video_server_port: i32,

    /// IPv6 video server host.
    #[serde(rename = "vsshost6")]
    pub video_server_host_ipv6: String,
}
