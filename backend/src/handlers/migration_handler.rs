use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::{models::GCPCredentials, utils::TerraformFileGenerator};

#[derive(Deserialize)]
pub struct MigrationPayload {
    pub source_credentials: GCPCredentials,
    pub destination_credentials: GCPCredentials,
    pub terraform_code: String,
}

#[derive(Serialize)]
pub struct MigrationResponse {
    pub success: bool,
    pub terraform_files_path: String,
    pub message: String,
}

pub async fn migrate_resources(
    Json(payload): Json<MigrationPayload>,
) -> Result<Json<MigrationResponse>, (StatusCode, String)> {
    println!(
        "Iniciando migração do projeto {} para {}",
        payload.source_credentials.project_id,
        payload.destination_credentials.project_id
    );

    // Substituir o project_id de origem pelo de destino no código Terraform
    let terraform_code = payload.terraform_code
        .replace(&payload.source_credentials.project_id, &payload.destination_credentials.project_id);

    // Gerar arquivos Terraform apenas para o projeto destino
    TerraformFileGenerator::create_terraform_files(
        &payload.destination_credentials.project_id,
        &terraform_code
    ).map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Erro ao gerar arquivos Terraform: {}", e)
    ))?;

    Ok(Json(MigrationResponse {
        success: true,
        terraform_files_path: format!("terraform_files/{}", payload.destination_credentials.project_id),
        message: format!("Arquivos Terraform gerados com sucesso para o projeto {}",
                         payload.destination_credentials.project_id).to_string(),
    }))
}