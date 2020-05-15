use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownResponse {
    pub s: i32, // TODO: What is s?
}