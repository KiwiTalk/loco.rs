use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectionInfo {
    #[serde(rename = "bgKeepItv")]
    pub bg_keep_itv: i32,
    #[serde(rename = "bgReconnItv")]
    pub bg_reconnection_itv: i32,
    #[serde(rename = "bgPingItv")]
    pub bg_ping_itv: i32,
    #[serde(rename = "fgPingItv")]
    pub fg_ping_itv: i32,
    pub ports: Vec<i32>,
    #[serde(rename = "connTimeout")]
    pub connection_timeout: i32,
    #[serde(rename = "recvHeaderTimeout")]
    pub receive_header_timeout: i32,
    #[serde(rename = "inSegTimeout")]
    pub in_seg_timeout: i32,
    #[serde(rename = "outSegTimeout")]
    pub out_seg_timeout: i32,
    #[serde(rename = "blockSendBufSize")]
    pub block_send_buf_size: i32,
}