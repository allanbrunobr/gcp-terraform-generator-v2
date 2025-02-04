}resource "google_storage_bucket" "gcf_v2_sources_58850674821_us_central1" {
resource "google_storage_bucket" "gcf_v2_sources_58850674821_us_east1" {
  name          = "gcf-v2-sources-58850674821-us-east1"
  location      = "US-EAST1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = true
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["GET"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "ARCHIVED"
      num_newer_versions = 3
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
resource "google_storage_bucket" "gcf_v2_uploads_58850674821_us_central1" {
  name          = "gcf-v2-uploads-58850674821-us-central1"
  location      = "US-CENTRAL1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = false
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["PUT"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "LIVE"
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
resource "google_storage_bucket" "gcf_v2_uploads_58850674821_us_east1" {
  name          = "gcf-v2-uploads-58850674821-us-east1"
  location      = "US-EAST1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = false
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["PUT"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "LIVE"
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
resource "google_storage_bucket" "gcf_v2_uploads_58850674821.us_central1.cloudfunctions.appspot.com" {
  name          = "gcf-v2-uploads-58850674821.us-central1.cloudfunctions.appspot.com"
  location      = "US-CENTRAL1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = false
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["PUT"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "LIVE"
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
resource "google_storage_bucket" "gcf_v2_uploads_58850674821.us_east1.cloudfunctions.appspot.com" {
  name          = "gcf-v2-uploads-58850674821.us-east1.cloudfunctions.appspot.com"
  location      = "US-EAST1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = false
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["PUT"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "LIVE"
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
resource "google_storage_bucket" "gen_ai_transcriptor_fiscalianuevaleon" {
  name          = "gen-ai-transcriptor-fiscalianuevaleon"
  location      = "US-EAST1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "enforced"

  versioning {
    enabled = false
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
}
resource "google_storage_bucket" "run_sources_mx_fiscalianuevoleon_cld_01_us_east1" {
  name          = "run-sources-app-springboot-project-us-east1"
  location      = "US-EAST1"
  storage_class = "STANDARD"
  force_destroy = true

  uniform_bucket_level_access = true
  public_access_prevention   = "inherited"

  versioning {
    enabled = true
  }
  cors {
    origin          = ["https://*.cloud.google.com","https://*.corp.google.com","https://*.corp.google.com:*","https://*.cloud.google","https://*.byoid.goog"]
    method          = ["GET"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      with_state = "ARCHIVED"
      num_newer_versions = 3
    }
  }
  soft_delete_policy {
    retention_duration_seconds = 604800
  }
  labels = {
    goog-managed-by = "cloudfunctions"
  }
}
