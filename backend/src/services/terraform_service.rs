use crate::models::*;
use crate::services::formatters::TerraformFormatter;
use crate::services::constants::*;

pub struct TerraformService;

impl crate::services::TerraformService {
    pub fn generate_provider_code(project_id: &str) -> String {
        format!(
            r#"# Arquivo gerado automaticamente
provider "google" {{
  project     = "{}"
  credentials = "credentials.json"
}}
"#,
            project_id
        )
    }

    pub fn generate_vpc_code(vpc: &GCPResource) -> String {
        let details = &vpc.details;

        let mut vpc_code = format!(
            r#"
resource "google_compute_network" "{}" {{
  name                                      = "{}"
  description                               = "{}"
  auto_create_subnetworks                   = {}
  routing_mode                              = "{}"
  network_firewall_policy_enforcement_order = "{}"
  bgp_best_path_selection_mode              = "{}""#,
            vpc.name,
            vpc.name,
            details["description"].as_str().unwrap_or(""),
            details["autoCreateSubnetworks"].as_bool().unwrap_or(false),
            details["routingConfig"]["routingMode"]
                .as_str()
                .unwrap_or("REGIONAL"),
            details["networkFirewallPolicyEnforcementOrder"]
                .as_str()
                .unwrap_or("AFTER_CLASSIC_FIREWALL"),
            details["routingConfig"]["bgpBestPathSelectionMode"]
                .as_str()
                .unwrap_or("LEGACY")
        );

        if let Some(peerings) = details["peerings"].as_array() {
            for peering in peerings {
                vpc_code.push_str(&format!(
                    r#"

  peering {{
    name                                = "{}"
    network                             = "{}"
    export_custom_routes                = {}
    import_custom_routes                = {}
    export_subnet_routes_with_public_ip = {}
    import_subnet_routes_with_public_ip = {}"#,
                    peering["name"].as_str().unwrap_or(""),
                    peering["network"].as_str().unwrap_or(""),
                    peering["exportCustomRoutes"].as_bool().unwrap_or(false),
                    peering["importCustomRoutes"].as_bool().unwrap_or(false),
                    peering["exportSubnetRoutesWithPublicIp"]
                        .as_bool()
                        .unwrap_or(true),
                    peering["importSubnetRoutesWithPublicIp"]
                        .as_bool()
                        .unwrap_or(true)
                ));
                vpc_code.push_str("\n  }");
            }
        }
        vpc_code.push_str("\n}\n");

        vpc_code
    }

    pub fn generate_cloud_run_code(cloud_run: &CloudRun) -> String {
        format!(
            r#"
resource "google_cloud_run_service" "{}" {{
  name     = "{}"
  location = "{}"

  template {{
    metadata {{
      annotations = {{
        "autoscaling.knative.dev/maxScale" = "{}"
{}
      }}
    }}

    spec {{
      container_concurrency = {}
      service_account_name = "{}"

      containers {{
        image = "{}"

        resources {{
          limits = {{
            cpu    = "{}"
            memory = "{}"
          }}
        }}

        ports {{
          container_port = 8080
          name          = "http1"
        }}

        env {{
          name  = "LOG_EXECUTION_ID"
          value = "true"
        }}

        startup_probe {{
          failure_threshold     = 1
          period_seconds       = 240
          timeout_seconds     = 240
          tcp_socket {{
            port = 8080
          }}
        }}
      }}
    }}
  }}

  traffic {{
    percent         = 100
    latest_revision = true
  }}
}}

# IAM - Permitir acesso público (noauth)
data "google_iam_policy" "noauth" {{
  binding {{
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }}
}}

resource "google_cloud_run_service_iam_policy" "{}_noauth" {{
  location = {}
  project  = {}
  service  = google_cloud_run_service.{}.name

  policy_data = data.google_iam_policy.noauth.policy_data
}}"#,
            cloud_run.name,
            cloud_run.name,
            cloud_run.location,
            cloud_run.max_scale.as_ref().unwrap_or(&"100".to_string()),
            if let Some(min_scale) = &cloud_run.min_scale {
                format!(
                    "        \"run.googleapis.com/minScale\" = \"{}\"",
                    min_scale
                )
            } else {
                "".to_string()
            },
            cloud_run.container_concurrency,
            cloud_run.service_account,
            cloud_run.image,
            cloud_run.cpu,
            cloud_run.memory,
            cloud_run.name,
            cloud_run.location,
            "var.project_id",
            cloud_run.name
        )
    }

    pub fn generate_function_code(function: &CloudFunction) -> String {
        format!(
            r#"
resource "google_cloudfunctions2_function" "{}" {{
  name        = "{}"
  location    = "{}"
  description = "Managed by Terraform"

  build_config {{
    runtime     = "{}"
    entry_point = "{}"
    source {{
      storage_source {{
        bucket = "gcf-v2-sources-{}-{}"
        object = "{}/function-source.zip"
      }}
    }}
  }}

  service_config {{
    max_instance_count             = {}
    min_instance_count            = 1
    available_memory              = "{}"
    timeout_seconds               = {}
    service_account_email         = "{}"
    environment_variables         = {{
      LOG_EXECUTION_ID = "true"
    }}
    ingress_settings             = "ALLOW_ALL"
    all_traffic_on_latest_revision = true
  }}

  traffic {{
    percent         = 100
    latest_revision = true
  }}
}}

# IAM - Allow unauthenticated access
data "google_iam_policy" "noauth_{}" {{
  binding {{
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }}
}}

resource "google_cloudfunctions2_function_iam_policy" "{}_noauth" {{
  project     = {}
  location    = "{}"
  cloud_function = google_cloudfunctions2_function.{}.name
  policy_data = data.google_iam_policy.noauth_{}.policy_data
}}
"#,
            function.name,                                   // nome da função
            function.name,                                   // nome
            function.region,                                 // localização
            function.runtime,                                // runtime (python312, nodejs, etc)
            function.entry_point,                            // entry point
            function.project_id,                             // project id para bucket
            function.region,                                 // região para bucket
            function.name,                                   // nome do objeto no bucket
            10,                                              // max_instance_count padrão
            "1Gi",                                           // memória disponível
            60,                                              // timeout em segundos
            "service-account@developer.gserviceaccount.com", // service account email
            function.name,                                   // nome para policy
            function.name,                                   // nome para iam policy
            "var.project_id",                                // variável do projeto
            function.region,                                 // região
            function.name,                                   // nome da função
            function.name                                    // nome para policy
        )
    }

    pub fn generate_bucket_code(bucket: &GCSBucket) -> String {
        let mut code = format!(
            r#"resource "google_storage_bucket" "{}" {{
  name          = "{}"
  location      = "{}"
  storage_class = "{}"
  force_destroy = true

  uniform_bucket_level_access = {}
  public_access_prevention   = "{}"
"#,
            bucket.name.replace("-", "_"),
            bucket.name,
            bucket.location,
            bucket.storage_class,
            bucket.uniform_bucket_level_access,
            bucket.public_access_prevention
        );

        if let Some(versioning) = &bucket.versioning {
            code.push_str(&format!(
                r#"
  versioning {{
    enabled = {}
  }}"#,
                versioning.enabled
            ));
        }

        if let Some(cors_configs) = &bucket.cors {
            for cors in cors_configs {
                code.push_str(&format!(
                    r#"
  cors {{
    origin          = {}
    method          = {}
    response_header = {}
    max_age_seconds = {}
  }}"#,
                    serde_json::to_string(&cors.origin).unwrap(),
                    serde_json::to_string(&cors.method).unwrap(),
                    serde_json::to_string(&cors.response_header).unwrap(),
                    cors.max_age_seconds
                ));
            }
        }

        if let Some(rules) = &bucket.lifecycle_rule {
            for rule in rules {
                code.push_str(&format!(
                    r#"
  lifecycle_rule {{
    action {{
      type = "{}"{}
    }}
    condition {{"#,
                    rule.action.r#type,
                    if let Some(sc) = &rule.action.storage_class {
                        format!("\n      storage_class = \"{}\"", sc)
                    } else {
                        "".to_string()
                    }
                ));

                let condition = &rule.condition;
                if let Some(age) = condition.age {
                    code.push_str(&format!("\n      age = {}", age));
                }
                if let Some(created_before) = &condition.created_before {
                    code.push_str(&format!("\n      created_before = \"{}\"", created_before));
                }
                if let Some(with_state) = &condition.with_state {
                    code.push_str(&format!("\n      with_state = \"{}\"", with_state));
                }
                if let Some(storage_classes) = &condition.matches_storage_class {
                    code.push_str(&format!(
                        "\n      matches_storage_class = {}",
                        serde_json::to_string(&storage_classes).unwrap()
                    ));
                }
                if let Some(versions) = condition.num_newer_versions {
                    code.push_str(&format!("\n      num_newer_versions = {}", versions));
                }
                if let Some(days) = condition.days_since_noncurrent_time {
                    code.push_str(&format!("\n      days_since_noncurrent_time = {}", days));
                }
                if let Some(time) = &condition.custom_time_before {
                    code.push_str(&format!("\n      custom_time_before = \"{}\"", time));
                }
                if let Some(days) = condition.days_since_custom_time {
                    code.push_str(&format!("\n      days_since_custom_time = {}", days));
                }
                code.push_str("\n    }\n  }");
            }
        }

        // Website configuration
        if let Some(website) = &bucket.website {
            code.push_str("\n  website {");
            if let Some(main_page) = &website.main_page_suffix {
                code.push_str(&format!("\n    main_page_suffix = \"{}\"", main_page));
            }
            if let Some(not_found) = &website.not_found_page {
                code.push_str(&format!("\n    not_found_page = \"{}\"", not_found));
            }
            code.push_str("\n  }");
        }

        // Retention Policy
        if let Some(retention) = &bucket.retention_policy {
            code.push_str(&format!(
                r#"
  retention_policy {{
    is_locked = {}
    retention_period = {}
  }}"#,
                retention.is_locked, retention.retention_period
            ));
        }

        // Logging configuration
        if let Some(logging) = &bucket.logging {
            code.push_str(&format!(
                r#"
  logging {{
    log_bucket = "{}"{}"#,
                logging.log_bucket,
                if let Some(prefix) = &logging.log_object_prefix {
                    format!("\n    log_object_prefix = \"{}\"", prefix)
                } else {
                    "".to_string()
                }
            ));
            code.push_str("\n  }");
        }

        // Encryption configuration
        if let Some(encryption) = &bucket.encryption {
            code.push_str(&format!(
                r#"
  encryption {{
    default_kms_key_name = "{}"
  }}"#,
                encryption.default_kms_key_name
            ));
        }

        // Soft Delete Policy
        if let Some(policy) = &bucket.soft_delete_policy {
            code.push_str(&format!(
                r#"
  soft_delete_policy {{
    retention_duration_seconds = {}
  }}"#,
                policy.retention_duration_seconds
            ));
        }

        if let Some(labels) = &bucket.labels {
            code.push_str("\n  labels = {\n");
            for (key, value) in labels {
                code.push_str(&format!("    {} = \"{}\"\n", key, value));
            }
            code.push_str("  }");
        }

        code.push_str("\n}\n");
        code
    }

    pub fn generate_pubsub_topic_code(topic: &PubSubTopic) -> String {
        let mut code = format!(
            r#"
resource "google_pubsub_topic" "{}" {{
  name    = "{}"
  project = "{}"

  message_retention_duration = "{}""#,
            topic.name.replace("-", "_"),
            topic.name,
            topic.project_id,
            topic.message_retention_duration,
        );

        if let Some(kms_key) = &topic.kms_key_name {
            code.push_str(&format!(
                r#"
  kms_key_name = "{}""#,
                kms_key
            ));
        }

        if let Some(labels) = &topic.labels {
            code.push_str("\n\n  labels = {");
            for (key, value) in labels {
                code.push_str(&format!(
                    r#"
    {} = "{}""#,
                    key, value
                ));
            }
            code.push_str("\n  }");
        }

        if let Some(storage_policy) = &topic.message_storage_policy {
            code.push_str("\n\n  message_storage_policy {");
            code.push_str(&format!(
                r#"
    allowed_persistence_regions = {}
  "#,
                serde_json::to_string(&storage_policy.allowed_persistence_regions).unwrap()
            ));
            code.push_str("}");
        }

        if let Some(schema) = &topic.schema_settings {
            code.push_str(&format!(
                r#"

  schema_settings {{
    schema   = "{}"
    encoding = "{}"
  }}"#,
                schema.schema, schema.encoding
            ));
        }

        code.push_str("\n}\n");
        code
    }

    pub fn generate_bigquery_dataset_code(dataset: &BigQueryDataset) -> String {
        let mut code = format!(
            r#"
resource "google_bigquery_dataset" "{}" {{
  dataset_id = "{}"
  project    = "{}"
  location   = "{}""#,
            dataset.dataset_id.replace("-", "_"),
            dataset.dataset_id,
            dataset.project_id,
            dataset.location,
        );

        if let Some(friendly_name) = &dataset.friendly_name {
            code.push_str(&format!(
                r#"
  friendly_name = "{}""#,
                friendly_name
            ));
        }

        if let Some(description) = &dataset.description {
            code.push_str(&format!(
                r#"
  description = "{}""#,
                description
            ));
        }

        if let Some(expiration) = dataset.default_table_expiration_ms {
            code.push_str(&format!(
                r#"
  default_table_expiration_ms = {}"#,
                expiration
            ));
        }

        if let Some(labels) = &dataset.labels {
            code.push_str("\n\n  labels = {");
            for (key, value) in labels {
                code.push_str(&format!(
                    r#"
    {} = "{}""#,
                    key, value
                ));
            }
            code.push_str("\n  }");
        }

        for access in &dataset.access {
            code.push_str("\n\n  access {");
            code.push_str(&format!(
                r#"
    role = "{}""#,
                access.role
            ));

            if let Some(user) = &access.user_by_email {
                code.push_str(&format!(
                    r#"
    user_by_email = "{}""#,
                    user
                ));
            }

            if let Some(group) = &access.group_by_email {
                code.push_str(&format!(
                    r#"
    group_by_email = "{}""#,
                    group
                ));
            }

            if let Some(domain) = &access.domain {
                code.push_str(&format!(
                    r#"
    domain = "{}""#,
                    domain
                ));
            }

            if let Some(special_group) = &access.special_group {
                code.push_str(&format!(
                    r#"
    special_group = "{}""#,
                    special_group
                ));
            }

            code.push_str("\n  }");
        }

        if let Some(encryption) = &dataset.encryption_configuration {
            code.push_str(&format!(
                r#"

  encryption_configuration {{
    kms_key_name = "{}"
  }}"#,
                encryption.kms_key_name
            ));
        }

        code.push_str("\n}\n");
        code
    }

    pub fn generate_pubsub_subscription_code(sub: &PubSubSubscription) -> String {
        let mut code = format!(
            r#"
resource "google_pubsub_subscription" "{}" {{
  name  = "{}"
  topic = "{}"

  ack_deadline_seconds = {}
  message_retention_duration = "{}""#,
            sub.name.replace("-", "_"),
            sub.name,
            sub.topic,
            sub.ack_deadline_seconds,
            sub.message_retention_duration,
        );

        if let Some(expiration_policy) = &sub.expiration_policy {
            code.push_str(&format!(
                r#"

  expiration_policy {{
    ttl = "{}"
  }}"#,
                expiration_policy.ttl
            ));
        }

        if let Some(push_config) = &sub.push_config {
            if !push_config.push_endpoint.is_empty() {
                code.push_str(&format!(
                    r#"

  push_config {{
    push_endpoint = "{}""#,
                    push_config.push_endpoint
                ));

                if let Some(attributes) = &push_config.attributes {
                    code.push_str("\n    attributes = {");
                    for (key, value) in attributes {
                        code.push_str(&format!(
                            r#"
      {} = "{}""#,
                            key, value
                        ));
                    }
                    code.push_str("\n    }");
                }

                code.push_str("\n  }");
            }
        }

        if sub.retain_acked_messages {
            code.push_str(&r#"
  retain_acked_messages = true"#.to_string());
        }

        if sub.enable_message_ordering {
            code.push_str(&r#"
  enable_message_ordering = true"#.to_string());
        }

        code.push_str("\n}\n");
        code
    }
}
