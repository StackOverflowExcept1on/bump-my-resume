use super::{request::*, response::*, Error, Result};

use std::fmt::Display;

/// Provides access to Headhunter's API using the user-defined `Request` and `Response` structures
pub struct Client {
    access_token: String,
    reusable_client: reqwest::Client,
}

impl Client {
    const USER_AGENT: &'static str = "App/1.0 (hidden@gmail.com)";
    const BASE_URL: &'static str = "https://api.hh.ru";

    /// Creates new `Client` instance with `access_token` (token doesn't checked)
    pub fn new(access_token: String) -> Result<Self> {
        Ok(Self {
            access_token,
            reusable_client: reqwest::ClientBuilder::new()
                .user_agent(Self::USER_AGENT)
                .build()?,
        })
    }

    #[inline]
    fn build_url<Req: Request>() -> Result<String> {
        let req = Req::method().ok_or_else(|| Error::UrlBuild)?;
        Ok(format!("{url}/{req}", url = Self::BASE_URL))
    }

    #[inline]
    fn build_url_with_value<Req: Request, T: Display>(value: T) -> Result<String> {
        let req = Req::build_url(value).ok_or_else(|| Error::UrlBuild)?;
        Ok(format!("{url}/{req}", url = Self::BASE_URL))
    }

    async fn request_base<Url: reqwest::IntoUrl, Req: Request>(
        &self,
        method: reqwest::Method,
        url: Url,
        req: &Req,
    ) -> Result<reqwest::Response> {
        let response = self
            .reusable_client
            .request(method, url)
            .json(req)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        Ok(response)
    }

    async fn request<Url: reqwest::IntoUrl, Req: Request>(
        &self,
        method: reqwest::Method,
        url: Url,
        req: &Req,
    ) -> Result<Req::Response> {
        let ret = self.request_base(method, url, req).await?;

        if ret.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::from_str::<'_, Req::Response>("null")?);
        }

        let response = ret.json::<HeadhunterResponse<Req::Response>>().await?;
        match response {
            HeadhunterResponse::Response(response) => Ok(response),
            HeadhunterResponse::Error(error) => Err(Error::Headhunter(error)),
        }
    }

    /// Interacts with API using the GET method, you need to pass `&Request` structure
    pub async fn get<Req: Request>(&self, req: &Req) -> Result<Req::Response> {
        let url = Self::build_url::<Req>()?;
        self.request(reqwest::Method::GET, url, req).await
    }

    /// Interacts with API using the POST method, you need to pass `&Request` structure and `value`
    pub async fn post_with_value<Req: Request, T: Display>(
        &self,
        req: &Req,
        value: T,
    ) -> Result<Req::Response> {
        let url = Self::build_url_with_value::<Req, _>(value)?;
        self.request(reqwest::Method::POST, url, req).await
    }
}
