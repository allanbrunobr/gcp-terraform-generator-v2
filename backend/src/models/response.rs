use serde::Serialize;

#[derive(Serialize)]
pub struct TerraformResponse {
    pub resources: Vec<crate::models::GCPResource>,
    pub cloud_functions: Vec<crate::models::CloudFunction>,
    pub cloud_runs: Vec<crate::models::CloudRun>,
    pub storage_buckets: Vec<crate::models::GCSBucket>,
    pub pubsub_topics: Vec<crate::models::PubSubTopic>,
    pub pubsub_subscriptions: Vec<crate::models::PubSubSubscription>,
    pub bigquery_datasets: Vec<crate::models::BigQueryDataset>,
    pub terraform_code: String,
    pub terraform_files_path: String,
}
