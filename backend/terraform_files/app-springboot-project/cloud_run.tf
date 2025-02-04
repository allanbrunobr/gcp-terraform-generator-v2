resource "google_cloud_run_service" "transcriptor-nl-backend" {
  name     = "transcriptor-nl-backend"
  location = "us-central1"

  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "100"
        "run.googleapis.com/minScale" = "1"
      }
    }

    spec {
      container_concurrency = 80
      service_account_name = "58850674821-compute@developer.gserviceaccount.com"

      containers {
        image = "us-central1-docker.pkg.dev/app-springboot-project/cloud-run-source-deploy/transcriptor-backend/transcriptor-nl-backend:619a1f60aabb77d6061dcd1ac728426dc3816a5b"

        resources {
          limits = {
            cpu    = "1000m"
            memory = "1Gi"
          }
        }

        ports {
          container_port = 8080
          name          = "http1"
        }

        env {
          name  = "LOG_EXECUTION_ID"
          value = "true"
        }

        startup_probe {
          failure_threshold     = 1
          period_seconds       = 240
          timeout_seconds     = 240
          tcp_socket {
            port = 8080
          }
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
resource "google_cloud_run_service" "transcriptor-nuevaleonbackend" {
  name     = "transcriptor-nuevaleonbackend"
  location = "us-central1"

  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "100"

      }
    }

    spec {
      container_concurrency = 80
      service_account_name = "58850674821-compute@developer.gserviceaccount.com"

      containers {
        image = "us-central1-docker.pkg.dev/app-springboot-project/cloud-run-source-deploy-staging/transcriptor-nuevaleonbackend-develop:e8f449aece36622ebb3669d9af102c0dc24e65c3"

        resources {
          limits = {
            cpu    = "1000m"
            memory = "512Mi"
          }
        }

        ports {
          container_port = 8080
          name          = "http1"
        }

        env {
          name  = "LOG_EXECUTION_ID"
          value = "true"
        }

        startup_probe {
          failure_threshold     = 1
          period_seconds       = 240
          timeout_seconds     = 240
          tcp_socket {
            port = 8080
          }
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
resource "google_cloud_run_service" "transcritor-fiscalia-leon" {
  name     = "transcritor-fiscalia-leon"
  location = "us-central1"

  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "100"
        "run.googleapis.com/minScale" = "1"
      }
    }

    spec {
      container_concurrency = 80
      service_account_name = "58850674821-compute@developer.gserviceaccount.com"

      containers {
        image = "us-central1-docker.pkg.dev/app-springboot-project/cloud-run-source-deploy/transcritor-fiscalia-leon/transcritor-fiscalia-leon:924dae050ce7768ca48172df3ab5197f4402fa6e"

        resources {
          limits = {
            cpu    = "1000m"
            memory = "1Gi"
          }
        }

        ports {
          container_port = 8080
          name          = "http1"
        }

        env {
          name  = "LOG_EXECUTION_ID"
          value = "true"
        }

        startup_probe {
          failure_threshold     = 1
          period_seconds       = 240
          timeout_seconds     = 240
          tcp_socket {
            port = 8080
          }
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
