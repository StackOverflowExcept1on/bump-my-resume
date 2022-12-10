use std::borrow::Cow;
use std::time::Duration;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use url::Url;

use super::{request::*, response::*, Error, Result};

pub struct ApplicationCredentials<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

pub struct UserCredentials<'a> {
    pub login: &'a str,
    pub password: &'a str,
}

pub struct AuthenticationClient<'a> {
    application: ApplicationCredentials<'a>,
    user: UserCredentials<'a>,
}

impl<'a> AuthenticationClient<'a> {
    const SERVER_URL: &'static str = "http://localhost:9515";
    const BASE_URL: &'static str = "https://hh.ru";

    pub fn new(application: ApplicationCredentials<'a>, user: UserCredentials<'a>) -> Self {
        Self { application, user }
    }

    pub async fn get_authorization_code(&self) -> Result<String> {
        let driver = WebDriver::new(Self::SERVER_URL, DesiredCapabilities::chrome()).await?;

        let request = UserAuthorizationRequest {
            response_type: "code",
            client_id: self.application.client_id,
        };

        let mut url = Url::parse(Self::BASE_URL)?;
        url.set_path(UserAuthorizationRequest::method().ok_or_else(|| Error::UrlBuild)?);

        let query = serde_urlencoded::to_string(request)?;
        url.set_query(Some(&query));

        driver.goto(url.as_str()).await?;

        let elem_form = driver
            .find(By::XPath("//*[@data-qa='account-login-form']"))
            .await?;

        let elem_login = elem_form
            .find(By::XPath("//*[@data-qa='login-input-username']"))
            .await?;
        let elem_password = elem_form
            .find(By::XPath("//*[@data-qa='login-input-password']"))
            .await?;

        let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;

        elem_login.send_keys(self.user.login).await?;
        elem_password.send_keys(self.user.password).await?;

        elem_button.click().await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let url = driver.current_url().await?;
        let code = url
            .query_pairs()
            .find(|(key, _)| key == &Cow::Borrowed("code"))
            .map(|(_, value)| value)
            .expect("could not find authorization_code")
            .to_string();

        driver.quit().await?;

        Ok(code)
    }

    pub async fn request<Req: Request>(&self, req: &Req) -> Result<Req::Response> {
        let mut url = Url::parse(Self::BASE_URL)?;
        url.set_path(Req::method().ok_or_else(|| Error::UrlBuild)?);

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .form(&req)
            .send()
            .await?
            .json::<Req::Response>()
            .await?;

        Ok(response)
    }

    pub async fn perform_authentication(
        &self,
        authorization_code: &str,
    ) -> Result<UserOpenAuthorizationResponse> {
        self.request(&UserOpenAuthorizationRequest {
            grant_type: "authorization_code",
            client_id: self.application.client_id,
            client_secret: self.application.client_secret,
            code: authorization_code,
        })
        .await
    }
}
