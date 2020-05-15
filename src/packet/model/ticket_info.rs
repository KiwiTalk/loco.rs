use serde::Deserialize;

#[derive(Deserialize)]
pub struct TicketInfo {
    pub lsl: Vec<String>,
    pub lsl6: Vec<String>,
}