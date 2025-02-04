export interface ServiceAccountCredentials {
  type: string;
  project_id: string;
  private_key_id: string;
  private_key: string;
  client_email: string;
  client_id: string;
  auth_uri: string;
  token_uri: string;
  auth_provider_x509_cert_url: string;
  client_x509_cert_url: string;
  universe_domain: string;
}

export interface GCPResource {
  name: string;
  resource_type: string;
  details: any;
}

export interface CloudFunction {
  name: string;
  status: string;
  runtime: string;
  region: string;
  entry_point: string;
  project_id: string;
}

export interface TerraformResponse {
  resources: GCPResource[];
  cloud_functions: CloudFunction[];
  cloud_runs: CloudRun[];
  storage_buckets: GCSBucket[];
  pubsub_topics: PubSubTopic[];
  pubsub_subscriptions: PubSubSubscription[];
  bigquery_datasets: BigQueryDataset[];
  terraform_code: string;
}

export interface CloudRun {
  name: string;
  location: string;
  status: string;
  url: string;
  image: string;
  cpu: string;
  memory: string;
  min_scale?: string;
  max_scale?: string;
  container_concurrency: number;
  service_account: string;
}

export interface PubSubTopic {
  name: string;
  project_id: string;
  message_retention_duration: string;
  kms_key_name?: string;
  labels?: Record<string, string>;
  message_storage_policy?: {
    allowed_persistence_regions: string[];
  };
  schema_settings?: {
    schema: string;
    encoding: string;
  };
  satisfies_pzs: boolean;
}

export interface BigQueryDataset {
  project_id: string;
  dataset_id: string;
  friendly_name?: string;
  description?: string;
  location: string;
  default_table_expiration_ms?: number;
  labels?: Record<string, string>;
  access: Array<{
    role: string;
    user_by_email?: string;
    group_by_email?: string;
    domain?: string;
    special_group?: string;
  }>;
  encryption_configuration?: {
    kms_key_name: string;
  };
}

export interface GCSBucket {
  name: string;
  location: string;
  storage_class: string;
  versioning?: {
    enabled: boolean;
  };
  website?: {
    main_page_suffix?: string;
    not_found_page?: string;
  };
  cors?: Array<{
    origin: string[];
    method: string[];
    response_header: string[];
    max_age_seconds: number;
  }>;
  lifecycle_rule?: Array<{
    action: {
      type: string;
      storage_class?: string;
    };
    condition: {
      age?: number;
      created_before?: string;
      with_state?: string;
      matches_storage_class?: string[];
      num_newer_versions?: number;
      days_since_noncurrent_time?: number;
      custom_time_before?: string;
      days_since_custom_time?: number;
    };
  }>;
  retention_policy?: {
    is_locked: boolean;
    retention_period: number;
  };
  logging?: {
    log_bucket: string;
    log_object_prefix?: string;
  };
  encryption?: {
    default_kms_key_name: string;
  };
  uniform_bucket_level_access: boolean;
  public_access_prevention: string;
  labels?: Record<string, string>;
  soft_delete_policy?: {
    retention_duration_seconds: string;
  };
}

export interface PubSubSubscription {
  name: string;
  topic: string;
  push_config?: {
    push_endpoint: string;
    attributes?: Record<string, string>;
  };
  ack_deadline_seconds: number;
  message_retention_duration: string;
  retain_acked_messages: boolean;
  enable_message_ordering: boolean;
  expiration_policy?: {
    ttl: string;
  };
  filter?: string;
  dead_letter_policy?: {
    dead_letter_topic: string;
    max_delivery_attempts: number;
  };
  retry_policy?: {
    minimum_backoff: string;
    maximum_backoff: string;
  };
}