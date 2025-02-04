variable "project_id" {
  description = "GCP Project ID"
  type        = string
  default     = "app-springboot-project"
}

variable "credentials_file" {
  description = "Path to the GCP credentials JSON file"
  type        = string
  default     = "credentials.json"
}

variable "region" {
  description = "Default GCP region"
  type        = string
  default     = "us-central1"
}
