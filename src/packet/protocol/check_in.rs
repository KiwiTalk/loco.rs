use serde::Deserialize;

#[derive(Deserialize)]
pub struct CheckInResponse {
    host: Option<String>,
    host6: Option<String>,
    cshost: Option<String>,
    cshost6: Option<String>,
    csport: i32,
    vsshost: Option<String>,
    vsshost6: Option<String>,
    vssport: i32,
}