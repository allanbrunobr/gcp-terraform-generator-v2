use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BigQueryDataset {
    pub project_id: String,
    pub dataset_id: String,
    pub friendly_name: Option<String>,
    pub description: Option<String>,
    pub location: String,
    pub default_table_expiration_ms: Option<i64>,
    pub labels: Option<HashMap<String, String>>,
    pub access: Vec<crate::models::DatasetAccess>,
    pub encryption_configuration: Option<crate::models::EncryptionConfiguration>,
}

#[derive(Debug, Serialize)]
pub struct DatasetAccess {
    pub role: String,
    pub user_by_email: Option<String>,
    pub group_by_email: Option<String>,
    pub domain: Option<String>,
    pub special_group: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EncryptionConfiguration {
    pub kms_key_name: String,
}