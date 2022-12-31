use serde::{Deserialize, Serialize};

/// Result-like type that can fallback into error if can't parse `T` with serde
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum HeadhunterResponse<T> {
    Response(T),
    Error(serde_json::Value),
}

/// This structure is used to store access tokens and to update them, saved as `response.json`
#[derive(Debug, Deserialize, Serialize)]
pub struct UserOpenAuthorizationResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}

/// User information, useful for checking token
#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub auth_type: String,
    pub email: String,
}

/// List of resumes that were listed on the site
#[derive(Debug, Deserialize)]
pub struct MineResumesResponse {
    pub found: u32,
    pub items: Vec<super::types::Resume>,
}
