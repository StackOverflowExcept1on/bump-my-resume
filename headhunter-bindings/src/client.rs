use super::{request::*, response::*, Error, Result};

use std::fmt::Display;

pub struct Client<'a> {
    access_token: &'a str,
    reusable_client: reqwest::Client,
}

impl<'a> Client<'a> {
    const USER_AGENT: &'static str = "App/1.0 (hidden@gmail.com)";
    const BASE_URL: &'static str = "https://api.hh.ru";

    pub fn new(access_token: &'a str) -> Result<Self> {
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
    fn build_url_with_param<Req: Request, T: Display>(value: T) -> Result<String> {
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
            .bearer_auth(self.access_token)
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

    pub async fn get<Req: Request>(&self, req: &Req) -> Result<Req::Response> {
        let url = Self::build_url::<Req>()?;
        self.request(reqwest::Method::GET, url, req).await
    }

    pub async fn post_with_value<Req: Request, T: Display>(
        &self,
        req: &Req,
        value: T,
    ) -> Result<Req::Response> {
        let url = Self::build_url_with_param::<Req, _>(value)?;
        self.request(reqwest::Method::POST, url, req).await
    }
}
