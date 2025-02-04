use crate::models::{
    BigQueryDataset, CloudFunction, CloudRun, Cors, DatasetAccess,
    EncryptionConfiguration, ExpirationPolicy, GCPResource, GCSBucket, LifecycleAction,
    LifecycleCondition, LifecycleRule, MessageStoragePolicy, PubSubSubscription, PubSubTopic,
    PushConfig, SchemaSettings, SoftDeletePolicy, Versioning,
};
use anyhow::Result;
use std::process::Command;

pub struct GCPService;

impl GCPService {
    pub async fn list_functions(project_id: &str) -> Result<Vec<CloudFunction>> {
        println!("Listando Cloud Functions...");
        let output = Self::execute_gcloud_command(&[
            "functions",
            "list",
            "--project",
            project_id,
            "--format=json",
        ])?;
        println!("Output do comando functions list: {}", output);

        let functions: Vec<serde_json::Value> = serde_json::from_str(&output)?;

        Ok(functions
            .into_iter()
            .filter_map(|func| {
                let name = func["name"].as_str()?;
                let state = func["state"].as_str()?;
                let runtime = func["buildConfig"]["runtime"].as_str()?;
                let region = if name.contains("/locations/") {
                    name.split("/locations/").nth(1)?.split("/").next()?
                } else {
                    "us-central1" // default
                };
                let entry_point = func["buildConfig"]["entryPoint"].as_str()?;

                println!(
                    "Processando function: {} {} {} {} {}",
                    name, state, runtime, region, entry_point
                );

                Some(CloudFunction {
                    name: name.split("/functions/").last()?.to_string(),
                    status: state.to_string(),
                    runtime: runtime.to_string(),
                    region: region.to_string(),
                    entry_point: entry_point.to_string(),
                    project_id: project_id.to_string(),
                })
            })
            .collect())
    }

    pub async fn list_vpcs(project_id: &str) -> Result<Vec<GCPResource>> {
        let output = Self::execute_gcloud_command(&[
            "compute",
            "networks",
            "list",
            "--project",
            project_id,
            "--format=json",
        ])?;

        let vpcs: Vec<serde_json::Value> = serde_json::from_str(&output)?;
        let mut detailed_vpcs = Vec::new();

        for vpc in vpcs {
            if let Some(name) = vpc["name"].as_str() {
                println!("Obtendo detalhes da VPC: {}", name);

                let detail_output = Self::execute_gcloud_command(&[
                    "compute",
                    "networks",
                    "describe",
                    name,
                    "--project",
                    project_id,
                    "--format=json",
                ])?;

                let vpc_details: serde_json::Value = serde_json::from_str(&detail_output)?;

                detailed_vpcs.push(GCPResource {
                    name: name.to_string(),
                    resource_type: "google_compute_network".to_string(),
                    details: vpc_details,
                });
            }
        }

        Ok(detailed_vpcs)
    }

    pub async fn list_cloud_runs(project_id: &str) -> Result<Vec<CloudRun>> {
        println!("Listando Cloud Run services...");
        let output = Self::execute_gcloud_command(&[
            "run",
            "services",
            "list",
            "--project",
            project_id,
            "--format=json",
        ])?;

        println!("Output do comando cloud run list: {}", output);

        let services: Vec<serde_json::Value> = serde_json::from_str(&output)?;

        Ok(services
            .into_iter()
            .filter(|svc| {
                // Verifica se NÃO é uma Cloud Function
                !svc["metadata"]["annotations"]
                    .as_object()
                    .map(|annotations| {
                        annotations.contains_key("cloudfunctions.googleapis.com/function-id")
                    })
                    .unwrap_or(false)
            })
            .filter_map(|svc| {
                let metadata = &svc["metadata"];
                let spec = &svc["spec"]["template"]["spec"];
                let containers = &spec["containers"][0];
                let status = &svc["status"];

                Some(CloudRun {
                    name: metadata["name"].as_str()?.to_string(),
                    location: metadata["labels"]["cloud.googleapis.com/location"]
                        .as_str()?
                        .to_string(),
                    status: status["conditions"][0]["status"].as_str()?.to_string(),
                    url: status["url"].as_str()?.to_string(),
                    image: containers["image"].as_str()?.to_string(),
                    cpu: containers["resources"]["limits"]["cpu"]
                        .as_str()?
                        .to_string(),
                    memory: containers["resources"]["limits"]["memory"]
                        .as_str()?
                        .to_string(),
                    min_scale: metadata["annotations"]["run.googleapis.com/minScale"]
                        .as_str()
                        .map(|s| s.to_string()),
                    max_scale: spec["metadata"]["annotations"]["autoscaling.knative.dev/maxScale"]
                        .as_str()
                        .map(|s| s.to_string()),
                    container_concurrency: spec["containerConcurrency"].as_i64()? as i32,
                    service_account: spec["serviceAccountName"].as_str()?.to_string(),
                })
            })
            .collect())
    }

    pub async fn list_buckets(project_id: &str) -> Result<Vec<GCSBucket>> {
        println!("Listando Cloud Storage Buckets...");
        let output = Self::execute_gcloud_command(&[
            "storage",
            "buckets",
            "list",
            "--project",
            project_id,
            "--format=json",
        ])?;

        let buckets: Vec<serde_json::Value> = serde_json::from_str(&output)?;

        Ok(buckets
            .into_iter()
            .map(|bucket| {
                GCSBucket {
                    name: bucket["name"].as_str().unwrap_or_default().to_string(),
                    location: bucket["location"].as_str().unwrap_or_default().to_string(),
                    storage_class: bucket["default_storage_class"]
                        .as_str()
                        .unwrap_or("STANDARD")
                        .to_string(),
                    versioning: Some(Versioning {
                        enabled: bucket["versioning_enabled"].as_bool().unwrap_or(false),
                    }),
                    website: None, // Adicionar parser se necessário
                    cors: bucket["cors_config"].as_array().map(|cors_array| {
                        cors_array
                            .iter()
                            .map(|cors| Cors {
                                origin: cors["origin"]
                                    .as_array()
                                    .unwrap_or(&Vec::new())
                                    .iter()
                                    .filter_map(|o| o.as_str().map(String::from))
                                    .collect(),
                                method: cors["method"]
                                    .as_array()
                                    .unwrap_or(&Vec::new())
                                    .iter()
                                    .filter_map(|m| m.as_str().map(String::from))
                                    .collect(),
                                response_header: vec!["*".to_string()],
                                max_age_seconds: 3600,
                            })
                            .collect()
                    }),
                    lifecycle_rule: bucket["lifecycle_config"]["rule"].as_array().map(|rules| {
                        rules
                            .iter()
                            .map(|rule| LifecycleRule {
                                action: LifecycleAction {
                                    r#type: rule["action"]["type"]
                                        .as_str()
                                        .unwrap_or("Delete")
                                        .to_string(),
                                    storage_class: None,
                                },
                                condition: LifecycleCondition {
                                    age: None,
                                    created_before: None,
                                    with_state: if rule["condition"]["isLive"]
                                        .as_bool()
                                        .unwrap_or(true)
                                    {
                                        Some("LIVE".to_string())
                                    } else {
                                        Some("ARCHIVED".to_string())
                                    },
                                    matches_storage_class: None,
                                    num_newer_versions: rule["condition"]["numNewerVersions"]
                                        .as_i64()
                                        .map(|n| n as i32),
                                    days_since_noncurrent_time: None,
                                    custom_time_before: None,
                                    days_since_custom_time: None,
                                },
                            })
                            .collect()
                    }),
                    retention_policy: None, // Adicionar parser se necessário
                    logging: None,          // Adicionar parser se necessário
                    encryption: None,       // Adicionar parser se necessário
                    uniform_bucket_level_access: bucket["uniform_bucket_level_access"]
                        .as_bool()
                        .unwrap_or(true),
                    public_access_prevention: bucket["public_access_prevention"]
                        .as_str()
                        .unwrap_or("enforced")
                        .to_string(),
                    labels: bucket["labels"].as_object().map(|labels| {
                        labels
                            .iter()
                            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
                            .collect()
                    }),
                    soft_delete_policy: bucket["soft_delete_policy"].as_object().map(|policy| {
                        SoftDeletePolicy {
                            retention_duration_seconds: policy["retentionDurationSeconds"]
                                .as_str()
                                .unwrap_or("604800")
                                .to_string(),
                        }
                    }),
                }
            })
            .collect())
    }

    pub async fn list_pubsub_topics(project_id: &str) -> Result<Vec<PubSubTopic>> {
        println!("Listando PubSub Topics...");
        let output = Self::execute_gcloud_command(&[
            "pubsub",
            "topics",
            "list",
            "--project",
            project_id,
            "--format=json",
        ])?;

        let topics: Vec<serde_json::Value> = serde_json::from_str(&output)?;

        Ok(topics
            .into_iter()
            .filter_map(|topic| {
                let name = topic["name"].as_str()?;
                let retention = topic["messageRetentionDuration"]
                    .as_str()
                    .unwrap_or("86600s"); // default 24h

                Some(PubSubTopic {
                    name: name.split("/topics/").last()?.to_string(),
                    project_id: project_id.to_string(),
                    message_retention_duration: retention.to_string(),
                    kms_key_name: topic["kmsKeyName"].as_str().map(String::from),
                    labels: topic["labels"].as_object().map(|labels| {
                        labels
                            .iter()
                            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
                            .collect()
                    }),
                    message_storage_policy: topic["messageStoragePolicy"]
                        ["allowedPersistenceRegions"]
                        .as_array()
                        .map(|regions| MessageStoragePolicy {
                            allowed_persistence_regions: regions
                                .iter()
                                .filter_map(|r| r.as_str().map(String::from))
                                .collect(),
                        }),
                    schema_settings: topic["schemaSettings"].as_object().map(|settings| {
                        SchemaSettings {
                            schema: settings["schema"].as_str().unwrap_or_default().to_string(),
                            encoding: settings["encoding"]
                                .as_str()
                                .unwrap_or_default()
                                .to_string(),
                        }
                    }),
                    satisfies_pzs: topic["satisfiesPzs"].as_bool().unwrap_or(false),
                })
            })
            .collect())
    }

    pub async fn list_bigquery_datasets(project_id: &str) -> Result<Vec<BigQueryDataset>> {
        println!("Listando BigQuery Datasets...");

        // Primeiro lista os datasets
        let output = Command::new("bq")
            .args(["ls", "--format=json", "--project_id", project_id])
            .output()?;

        let datasets: Vec<serde_json::Value> =
            serde_json::from_str(&String::from_utf8(output.stdout)?)?;

        println!("Datasets encontrados: {}", datasets.len());

        let mut detailed_datasets = Vec::new();

        // Para cada dataset, busca os detalhes
        for dataset in datasets {
            let dataset_id = dataset["datasetReference"]["datasetId"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Dataset ID não encontrado"))?;

            println!("Buscando detalhes do dataset: {}", dataset_id);

            // Busca detalhes do dataset específico
            let details_output = Command::new("bq")
                .args([
                    "show",
                    "--format=json",
                    &format!("{}:{}", project_id, dataset_id),
                ])
                .output()?;

            let details: serde_json::Value =
                serde_json::from_str(&String::from_utf8(details_output.stdout)?)?;

            detailed_datasets.push(BigQueryDataset {
                project_id: project_id.to_string(),
                dataset_id: dataset_id.to_string(),
                friendly_name: details["friendlyName"].as_str().map(String::from),
                description: details["description"].as_str().map(String::from),
                location: details["location"].as_str().unwrap_or("US").to_string(),
                default_table_expiration_ms: details["defaultTableExpirationMs"].as_i64(),
                labels: details["labels"].as_object().map(|labels| {
                    labels
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
                        .collect()
                }),
                access: details["access"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter_map(|access| {
                        Some(DatasetAccess {
                            role: access["role"].as_str()?.to_string(),
                            user_by_email: access["userByEmail"].as_str().map(String::from),
                            group_by_email: access["groupByEmail"].as_str().map(String::from),
                            domain: access["domain"].as_str().map(String::from),
                            special_group: access["specialGroup"].as_str().map(String::from),
                        })
                    })
                    .collect(),
                encryption_configuration: details["defaultEncryptionConfiguration"]["kmsKeyName"]
                    .as_str()
                    .map(|key| EncryptionConfiguration {
                        kms_key_name: key.to_string(),
                    }),
            });
        }

        println!("Total de datasets processados: {}", detailed_datasets.len());
        Ok(detailed_datasets)
    }

    pub async fn list_pubsub_subscriptions(project_id: &str) -> Result<Vec<PubSubSubscription>> {
        println!("Listando PubSub Subscriptions...");
        let output = Command::new("gcloud")
            .args([
                "pubsub",
                "subscriptions",
                "list",
                "--project",
                project_id,
                "--format=json",
            ])
            .output()?;

        let subscriptions: Vec<serde_json::Value> =
            serde_json::from_str(&String::from_utf8(output.stdout)?)?;

        println!("Subscriptions encontradas: {}", subscriptions.len());

        Ok(subscriptions
            .into_iter()
            .filter_map(|sub| {
                // Extrair o nome curto da subscription (remover o prefixo do projeto)
                let full_name = sub["name"].as_str()?;
                let name = full_name.split("/subscriptions/").last()?;

                // Extrair o nome curto do tópico
                let full_topic = sub["topic"].as_str()?;
                let topic = full_topic.split("/topics/").last()?;

                Some(PubSubSubscription {
                    name: name.to_string(),
                    topic: topic.to_string(),
                    push_config: sub["pushConfig"].as_object().map(|config| PushConfig {
                        push_endpoint: config
                            .get("pushEndpoint")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        attributes: config
                            .get("attributes")
                            .and_then(|attrs| attrs.as_object())
                            .map(|attrs| {
                                attrs
                                    .iter()
                                    .map(|(k, v)| {
                                        (k.clone(), v.as_str().unwrap_or_default().to_string())
                                    })
                                    .collect()
                            }),
                    }),
                    ack_deadline_seconds: sub["ackDeadlineSeconds"].as_i64().unwrap_or(10) as i32,
                    message_retention_duration: sub["messageRetentionDuration"]
                        .as_str()
                        .unwrap_or("604800s")
                        .to_string(),
                    retain_acked_messages: sub["retainAckedMessages"].as_bool().unwrap_or(false),
                    enable_message_ordering: sub["enableMessageOrdering"]
                        .as_bool()
                        .unwrap_or(false),
                    expiration_policy: sub["expirationPolicy"]["ttl"].as_str().map(|ttl| {
                        ExpirationPolicy {
                            ttl: ttl.to_string(),
                        }
                    }),
                    filter: sub["filter"].as_str().map(String::from),
                    dead_letter_policy: None, // Adicionando caso seja necessário no futuro
                    retry_policy: None,       // Adicionando caso seja necessário no futuro
                })
            })
            .collect())
    }

    fn execute_gcloud_command(args: &[&str]) -> Result<String> {
        let output = Command::new("gcloud").args(args).output()?;

        String::from_utf8(output.stdout)
            .map_err(|e| anyhow::anyhow!("Erro ao converter output: {}", e))
    }
}
