use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PubSubTopic {
    pub name: String,
    pub project_id: String,
    pub message_retention_duration: String,
    pub kms_key_name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub message_storage_policy: Option<crate::models::MessageStoragePolicy>,
    pub schema_settings: Option<crate::models::SchemaSettings>,
    pub satisfies_pzs: bool,
}

#[derive(Debug, Serialize)]
pub struct MessageStoragePolicy {
    pub allowed_persistence_regions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SchemaSettings {
    pub schema: String,
    pub encoding: String,
}

#[derive(Debug, Serialize)]
pub struct PubSubSubscription {
    pub name: String,
    pub topic: String,
    pub push_config: Option<crate::models::PushConfig>,
    pub ack_deadline_seconds: i32,
    pub message_retention_duration: String,
    pub retain_acked_messages: bool,
    pub enable_message_ordering: bool,
    pub expiration_policy: Option<crate::models::ExpirationPolicy>,
    pub filter: Option<String>,
    pub dead_letter_policy: Option<crate::models::DeadLetterPolicy>,
    pub retry_policy: Option<crate::models::RetryPolicy>,
}

#[derive(Debug, Serialize)]
pub struct PushConfig {
    pub push_endpoint: String,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct ExpirationPolicy {
    pub ttl: String,
}

#[derive(Debug, Serialize)]
pub struct DeadLetterPolicy {
    pub dead_letter_topic: String,
    pub max_delivery_attempts: i32,
}

#[derive(Debug, Serialize)]
pub struct RetryPolicy {
    pub minimum_backoff: String,
    pub maximum_backoff: String,
}
