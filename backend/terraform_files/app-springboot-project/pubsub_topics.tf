resource "google_pubsub_topic" "container_analysis_notes_v1beta1" {
  name    = "container-analysis-notes-v1beta1"
  project = "app-springboot-project"

  message_retention_duration = "86600s"
}
resource "google_pubsub_topic" "container_analysis_occurrences_v1beta1" {
  name    = "container-analysis-occurrences-v1beta1"
  project = "app-springboot-project"

  message_retention_duration = "86600s"
}
resource "google_pubsub_topic" "container_analysis_occurrences_v1" {
  name    = "container-analysis-occurrences-v1"
  project = "app-springboot-project"

  message_retention_duration = "86600s"
}
resource "google_pubsub_topic" "container_analysis_notes_v1" {
  name    = "container-analysis-notes-v1"
  project = "app-springboot-project"

  message_retention_duration = "86600s"
}
resource "google_pubsub_topic" "process_status_updates" {
  name    = "process-status-updates"
  project = "app-springboot-project"

  message_retention_duration = "86600s"
}
