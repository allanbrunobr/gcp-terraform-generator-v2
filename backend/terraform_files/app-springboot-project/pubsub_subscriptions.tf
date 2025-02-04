resource "google_pubsub_subscription" "dev_status_nuevaleon_subscription" {
  name  = "dev-status-nuevaleon-subscription"
  topic = "process-status-updates"

  ack_deadline_seconds = 10
  message_retention_duration = "604800s"

  expiration_policy {
    ttl = "2678400s"
  }
}
