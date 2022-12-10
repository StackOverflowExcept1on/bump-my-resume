pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Selenium error: {0}")]
    Selenium(#[from] thirtyfour::error::WebDriverError),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("URL serialization error: {0}")]
    UrlSerialize(#[from] serde_urlencoded::ser::Error),
    #[error("Headhunter API error: {0}")]
    Headhunter(serde_json::Value),
    #[error("URL build error")]
    UrlBuild,
}
