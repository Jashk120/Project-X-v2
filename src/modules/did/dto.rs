use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDidRequest {
    pub network: Option<String>,
    pub controller: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDidResponse {
    pub did: String,
    pub topic_id: String,
    pub public_key_base58: String,
    pub private_key_base58: String,
}

#[derive(Debug, Deserialize)]
pub struct ResolveDidRequest {
    pub did: String,
}
