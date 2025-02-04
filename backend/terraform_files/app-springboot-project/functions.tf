resource "google_cloudfunctions2_function" "gen-ai-audiences-bot-nl" {
  name        = "gen-ai-audiences-bot-nl"
  location    = "us-central1"
  description = "Managed by Terraform"

  build_config {
    runtime     = "python312"
    entry_point = "entry_point_http"
    source {
      storage_source {
        bucket = "gcf-v2-sources-app-springboot-project-us-central1"
        object = "gen-ai-audiences-bot-nl/function-source.zip"
      }
    }
  }

  service_config {
    max_instance_count             = 10
    min_instance_count            = 1
    available_memory              = "1Gi"
    timeout_seconds               = 60
    service_account_email         = "service-account@developer.gserviceaccount.com"
    environment_variables         = {
      LOG_EXECUTION_ID = "true"
    }
    ingress_settings             = "ALLOW_ALL"
    all_traffic_on_latest_revision = true
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
resource "google_cloudfunctions2_function" "gen-ai-denuncias" {
  name        = "gen-ai-denuncias"
  location    = "us-central1"
  description = "Managed by Terraform"

  build_config {
    runtime     = "python312"
    entry_point = "process_denuncia"
    source {
      storage_source {
        bucket = "gcf-v2-sources-app-springboot-project-us-central1"
        object = "gen-ai-denuncias/function-source.zip"
      }
    }
  }

  service_config {
    max_instance_count             = 10
    min_instance_count            = 1
    available_memory              = "1Gi"
    timeout_seconds               = 60
    service_account_email         = "service-account@developer.gserviceaccount.com"
    environment_variables         = {
      LOG_EXECUTION_ID = "true"
    }
    ingress_settings             = "ALLOW_ALL"
    all_traffic_on_latest_revision = true
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
resource "google_cloudfunctions2_function" "gen-ai-audiences-2" {
  name        = "gen-ai-audiences-2"
  location    = "us-east1"
  description = "Managed by Terraform"

  build_config {
    runtime     = "python312"
    entry_point = "process"
    source {
      storage_source {
        bucket = "gcf-v2-sources-app-springboot-project-us-east1"
        object = "gen-ai-audiences-2/function-source.zip"
      }
    }
  }

  service_config {
    max_instance_count             = 10
    min_instance_count            = 1
    available_memory              = "1Gi"
    timeout_seconds               = 60
    service_account_email         = "service-account@developer.gserviceaccount.com"
    environment_variables         = {
      LOG_EXECUTION_ID = "true"
    }
    ingress_settings             = "ALLOW_ALL"
    all_traffic_on_latest_revision = true
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
