use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct GCPResource {
    pub name: String,
    pub resource_type: String,
    pub details: serde_json::Value,
}






#[derive(Serialize, Debug)]
pub struct CloudFunction {
    pub name: String,
    pub status: String,
    pub runtime: String,
    pub region: String,
    pub entry_point: String,
    pub project_id: String,
}


#[derive(Serialize)]
pub struct CloudRun {
    pub name: String,
    pub location: String,
    pub status: String,
    pub url: String,
    pub image: String,
    pub cpu: String,
    pub memory: String,
    pub min_scale: Option<String>,
    pub max_scale: Option<String>,
    pub container_concurrency: i32,
    pub service_account: String,
}
#[derive(Debug, Serialize)]
pub struct GCSBucket {
    pub name: String,
    pub location: String,
    pub storage_class: String,
    pub versioning: Option<crate::models::Versioning>,
    pub website: Option<crate::models::Website>,
    pub cors: Option<Vec<crate::models::Cors>>,
    pub lifecycle_rule: Option<Vec<crate::models::LifecycleRule>>,
    pub retention_policy: Option<crate::models::RetentionPolicy>,
    pub logging: Option<crate::models::Logging>,
    pub encryption: Option<crate::models::Encryption>,
    pub uniform_bucket_level_access: bool,
    pub public_access_prevention: String,
    pub labels: Option<HashMap<String, String>>,
    pub soft_delete_policy: Option<crate::models::SoftDeletePolicy>,
}

#[derive(Debug, Serialize)]
pub struct Versioning {
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct Website {
    pub main_page_suffix: Option<String>,
    pub not_found_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Cors {
    pub origin: Vec<String>,
    pub method: Vec<String>,
    pub response_header: Vec<String>,
    pub max_age_seconds: i32,
}

#[derive(Debug, Serialize)]
pub struct LifecycleRule {
    pub action: crate::models::LifecycleAction,
    pub condition: crate::models::LifecycleCondition,
}

#[derive(Debug, Serialize)]
pub struct LifecycleAction {
    pub r#type: String,                // Delete, SetStorageClass, etc
    pub storage_class: Option<String>, // necessary if type for SetStorageClass
}

#[derive(Debug, Serialize)]
pub struct LifecycleCondition {
    pub age: Option<i32>,
    pub created_before: Option<String>,
    pub with_state: Option<String>, // "LIVE", "ARCHIVED", "ANY"
    pub matches_storage_class: Option<Vec<String>>,
    pub num_newer_versions: Option<i32>,
    pub days_since_noncurrent_time: Option<i32>,
    pub custom_time_before: Option<String>,
    pub days_since_custom_time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RetentionPolicy {
    pub is_locked: bool,
    pub retention_period: i32,
}

#[derive(Debug, Serialize)]
pub struct Logging {
    pub log_bucket: String,
    pub log_object_prefix: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Encryption {
    pub default_kms_key_name: String,
}

#[derive(Debug, Serialize)]
pub struct SoftDeletePolicy {
    pub retention_duration_seconds: String,
}