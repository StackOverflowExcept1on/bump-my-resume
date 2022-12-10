use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Resume {
    pub alternate_url: String,
}
