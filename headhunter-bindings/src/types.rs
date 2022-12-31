use serde::Deserialize;

/// Represents part of the resume object
#[derive(Debug, Deserialize)]
pub struct Resume {
    pub alternate_url: String,
}
