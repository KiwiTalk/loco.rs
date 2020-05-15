use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BuyCallServerResponse {
    cshost: Option<String>,
    cshost6: Option<String>,
    csport: i32,
    vsshost: Option<String>,
    vsshost6: Option<String>,
    vssport: i32,
}