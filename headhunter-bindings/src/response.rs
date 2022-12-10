use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum HeadhunterResponse<T> {
    Response(T),
    Error(serde_json::Value),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOpenAuthorizationResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub auth_type: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct MineResumesResponse {
    pub found: u32,
    pub items: Vec<super::types::Resume>,
}
