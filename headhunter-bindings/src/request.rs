use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

/// Trait for convenient work with different types of requests and simultaneous connection
/// with the response data type using the trait associated types
pub trait Request: Serialize {
    /// Response type that can be deserialized with serde
    type Response: DeserializeOwned;

    /// Returns string query to API to invoke current request
    fn method() -> Option<&'static str> {
        None
    }

    /// Builds an url with parameter for POST, PUT, DELETE requests
    fn build_url<T: Display>(_: T) -> Option<String> {
        None
    }
}

/// Request that is used to get the temporary code from the OAuth service
#[derive(Debug, Serialize)]
pub struct UserAuthorizationRequest<'a> {
    pub response_type: &'a str,
    pub client_id: &'a str,
}

impl<'a> Request for UserAuthorizationRequest<'a> {
    type Response = ();

    fn method() -> Option<&'static str> {
        Some("oauth/authorize")
    }
}

/// Request that is used to get access token from the temporary code
#[derive(Debug, Serialize)]
pub struct UserOpenAuthorizationRequest<'a> {
    pub grant_type: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub code: &'a str,
}

impl<'a> Request for UserOpenAuthorizationRequest<'a> {
    type Response = super::response::UserOpenAuthorizationResponse;

    fn method() -> Option<&'static str> {
        Some("oauth/token")
    }
}

/// Request that is used to renew access token
#[derive(Debug, Serialize)]
pub struct UserRenewOpenAuthorizationRequest<'a> {
    pub grant_type: &'a str,
    pub refresh_token: &'a str,
}

impl<'a> Request for UserRenewOpenAuthorizationRequest<'a> {
    type Response = super::response::UserOpenAuthorizationResponse;

    fn method() -> Option<&'static str> {
        Some("oauth/token")
    }
}

/// Request that can get user information, useful for checking token
#[derive(Debug, Serialize)]
pub struct MeRequest;

impl Request for MeRequest {
    type Response = super::response::MeResponse;

    fn method() -> Option<&'static str> {
        Some("me")
    }
}

/// Request that can get list of submitted resumes
#[derive(Debug, Serialize)]
pub struct MineResumesRequest;

impl Request for MineResumesRequest {
    type Response = super::response::MineResumesResponse;

    fn method() -> Option<&'static str> {
        Some("resumes/mine")
    }
}

/// Request that can publish new information in existing resume
#[derive(Debug, Serialize)]
pub struct PublishResumeRequest;

impl Request for PublishResumeRequest {
    type Response = ();

    fn build_url<T: Display>(resume_id: T) -> Option<String> {
        Some(format!("resumes/{resume_id}/publish"))
    }
}
