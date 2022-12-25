use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

pub trait Request: Serialize {
    type Response: DeserializeOwned;

    fn method() -> Option<&'static str> {
        None
    }

    fn build_url<T: Display>(_: T) -> Option<String> {
        None
    }
}

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

#[derive(Debug, Serialize)]
pub struct MeRequest;

impl Request for MeRequest {
    type Response = super::response::MeResponse;

    fn method() -> Option<&'static str> {
        Some("me")
    }
}

#[derive(Debug, Serialize)]
pub struct MineResumesRequest;

impl Request for MineResumesRequest {
    type Response = super::response::MineResumesResponse;

    fn method() -> Option<&'static str> {
        Some("resumes/mine")
    }
}

#[derive(Debug, Serialize)]
pub struct PublishResumeRequest;

impl Request for PublishResumeRequest {
    type Response = ();

    fn build_url<T: Display>(resume_id: T) -> Option<String> {
        Some(format!("resumes/{resume_id}/publish"))
    }
}
