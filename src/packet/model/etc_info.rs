#[derive(Deserialize)]
pub struct EtcInfo {
    #[serde(rename = "writeRetryTimeout")]
    pub write_retry_timeout: i32,
    #[serde(rename = "wakeLockTimeout")]
    pub wake_lock_timeout: i32,
    #[serde(rename = "tracerouteHost")]
    pub traceroute_host: Vec<String>,
    #[serde(rename = "tracerouteHost6")]
    pub traceroute_host6: Vec<String>,
}