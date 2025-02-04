provider "google" {
  project     = var.project_id
  credentials = var.credentials_file
  region      = var.region
}
