use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GCPCredentials {
    pub project_id: String,
    pub private_key: String,
    pub client_email: String,
    #[serde(rename = "type")]
    pub credential_type: String,
}
