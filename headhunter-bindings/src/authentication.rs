use std::borrow::Cow;
use std::time::Duration;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use url::Url;

use super::{request::*, response::*, Error, Result};

/// Represents the application's credentials, which can be obtained from https://dev.hh.ru
pub struct ApplicationCredentials<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

/// Represents user credentials for the http://hh.ru website
pub struct UserCredentials<'a> {
    pub login: &'a str,
    pub password: &'a str,
}

/// Helper structure for obtaining token using authorization
/// through credentials in Selenium - browser automation software
pub struct AuthenticationClient;

impl AuthenticationClient {
    const SERVER_URL: &'static str = "http://localhost:9515";
    const BASE_URL: &'static str = "https://hh.ru";

    /// Creates new `AuthenticationClient`
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Enters application and user data through Selenium to get temporary code
    pub async fn get_authorization_code(
        &self,
        application: &ApplicationCredentials<'_>,
        user: &UserCredentials<'_>,
    ) -> Result<String> {
        let driver = WebDriver::new(Self::SERVER_URL, DesiredCapabilities::chrome()).await?;

        let request = UserAuthorizationRequest {
            response_type: "code",
            client_id: application.client_id,
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

        elem_login.send_keys(user.login).await?;
        elem_password.send_keys(user.password).await?;

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

    /// Continues the authorization process, receives access token from the temporary code
    pub async fn perform_authentication(
        &self,
        application: &ApplicationCredentials<'_>,
        authorization_code: &str,
    ) -> Result<UserOpenAuthorizationResponse> {
        self.request(&UserOpenAuthorizationRequest {
            grant_type: "authorization_code",
            client_id: application.client_id,
            client_secret: application.client_secret,
            code: authorization_code,
        })
        .await
    }

    /// Creates an access token renewal request
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<UserOpenAuthorizationResponse> {
        self.request(&UserRenewOpenAuthorizationRequest {
            grant_type: "refresh_token",
            refresh_token,
        })
        .await
    }
}
