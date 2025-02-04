use crate::models::{GCPCredentials, TerraformResponse};
use crate::services::{GCPService, TerraformService};
use axum::{http::StatusCode, Json};
use crate::utils::TerraformFileGenerator;

pub async fn generate_terraform(
    Json(credentials): Json<GCPCredentials>,
) -> Result<Json<TerraformResponse>, (StatusCode, String)> {
    println!(
        "Iniciando geração do Terraform para projeto: {}",
        credentials.project_id
    );

    // Gerar código do provider
    let mut terraform_code = TerraformService::generate_provider_code(&credentials.project_id);

    // Listar VPCs
    let resources = GCPService::list_vpcs(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("VPCs encontradas: {}", resources.len());

    // Listar Cloud Functions
    let cloud_functions = GCPService::list_functions(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("Cloud Functions encontradas: {}", cloud_functions.len());

    // Listar Cloud Runs
    let cloud_runs = GCPService::list_cloud_runs(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("Cloud Runs encontrados: {}", cloud_runs.len());

    // Listar Storage Buckets
    let storage_buckets = GCPService::list_buckets(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("Storage Buckets encontrados: {}", storage_buckets.len());

    // Listar PubSub Topics
    let pubsub_topics = GCPService::list_pubsub_topics(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("PubSub Topics encontrados: {}", pubsub_topics.len());

    // Listar BigQuery Datasets
    let bigquery_datasets = GCPService::list_bigquery_datasets(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("BigQuery Datasets encontrados: {}", bigquery_datasets.len());

    let pubsub_subscriptions = GCPService::list_pubsub_subscriptions(&credentials.project_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!(
        "PubSub Subscriptions encontradas: {}",
        pubsub_subscriptions.len()
    );

    // Gerar código para cada recurso
    for vpc in &resources {
        terraform_code.push_str(&TerraformService::generate_vpc_code(vpc));
    }

    for function in &cloud_functions {
        terraform_code.push_str(&TerraformService::generate_function_code(function));
    }

    for cloud_run in &cloud_runs {
        terraform_code.push_str(&TerraformService::generate_cloud_run_code(cloud_run));
    }

    for bucket in &storage_buckets {
        terraform_code.push_str(&TerraformService::generate_bucket_code(bucket));
    }

    for topic in &pubsub_topics {
        terraform_code.push_str(&TerraformService::generate_pubsub_topic_code(topic));
    }

    for dataset in &bigquery_datasets {
        terraform_code.push_str(&TerraformService::generate_bigquery_dataset_code(dataset));
    }

    for subscription in &pubsub_subscriptions {
        terraform_code.push_str(&TerraformService::generate_pubsub_subscription_code(
            subscription,
        ));
    }
    // Gerar arquivos Terraform
    TerraformFileGenerator::create_terraform_files(&credentials.project_id, &terraform_code)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao gerar arquivos Terraform: {}", e)))?;



    Ok(Json(TerraformResponse {
        resources,
        cloud_functions,
        cloud_runs,
        storage_buckets,
        pubsub_topics,
        pubsub_subscriptions,
        bigquery_datasets,
        terraform_code,
        terraform_files_path: format!("terraform_files/{}", credentials.project_id),
    }))
}
