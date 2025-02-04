// src/utils/file_utils.rs
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct TerraformFileGenerator;

impl TerraformFileGenerator {
    pub fn create_terraform_files(project_id: &str, terraform_code: &str) -> Result<()> {
        // Criar diretório base se não existir
        let base_dir = Path::new("terraform_files").join(project_id);
        fs::create_dir_all(&base_dir)?;

        // Separar o código em diferentes arquivos
        Self::create_provider_file(&base_dir)?;
        Self::create_variables_file(&base_dir, project_id)?;
        Self::create_resource_files(&base_dir, terraform_code)?;

        Ok(())
    }

    fn create_provider_file(base_dir: &Path) -> Result<()> {
        let provider_code = r#"provider "google" {
  project     = var.project_id
  credentials = var.credentials_file
  region      = var.region
}
"#;
        fs::write(base_dir.join("provider.tf"), provider_code)?;
        Ok(())
    }

    fn create_variables_file(base_dir: &Path, project_id: &str) -> Result<()> {
        let variables_code = format!(r#"variable "project_id" {{
  description = "GCP Project ID"
  type        = string
  default     = "{}"
}}

variable "credentials_file" {{
  description = "Path to the GCP credentials JSON file"
  type        = string
  default     = "credentials.json"
}}

variable "region" {{
  description = "Default GCP region"
  type        = string
  default     = "us-central1"
}}
"#, project_id);
        fs::write(base_dir.join("variables.tf"), variables_code)?;
        Ok(())
    }

    fn create_resource_files(base_dir: &Path, terraform_code: &str) -> Result<()> {
        let resource_types = [
            ("google_compute_network", "networks"),
            ("google_cloudfunctions2_function", "functions"),
            ("google_cloud_run_service", "cloud_run"),
            ("google_storage_bucket", "storage"),
            ("google_pubsub_topic", "pubsub_topics"),
            ("google_pubsub_subscription", "pubsub_subscriptions"),
            ("google_bigquery_dataset", "bigquery"),
        ];

        for (resource_prefix, filename) in resource_types.iter() {
            let mut resource_code = String::new();
            let lines: Vec<&str> = terraform_code.lines().collect();
            let mut i = 0;

            while i < lines.len() {
                let line = lines[i];
                if line.contains(&format!("resource \"{}\"", resource_prefix)) {
                    let mut block_content = String::new();
                    block_content.push_str(line);
                    block_content.push('\n');

                    let mut brace_count = line.matches('{').count() as i32;
                    brace_count -= line.matches('}').count() as i32;

                    i += 1;

                    // Continue até que todas as chaves abertas sejam fechadas
                    while i < lines.len() && brace_count > 0 {
                        let current_line = lines[i];
                        block_content.push_str(current_line);
                        block_content.push('\n');

                        brace_count += current_line.matches('{').count() as i32;
                        brace_count -= current_line.matches('}').count() as i32;

                        i += 1;
                    }

                    // Garante que o bloco está fechado
                    if brace_count > 0 {
                        block_content.push_str("}\n\n");
                    }

                    resource_code.push_str(&block_content);
                } else {
                    i += 1;
                }
            }

            // Se encontrou recursos deste tipo, criar o arquivo
            if !resource_code.is_empty() {
                println!("Gerando arquivo {}.tf", filename);
                fs::write(base_dir.join(format!("{}.tf", filename)), resource_code)?;
            }
        }

        Ok(())
    }
}