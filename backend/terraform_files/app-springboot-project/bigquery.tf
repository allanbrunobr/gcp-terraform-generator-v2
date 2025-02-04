resource "google_bigquery_dataset" "gen_ai_audiences" {
  dataset_id = "gen_ai_audiences"
  project    = "app-springboot-project"
  location   = "us-east1"

  access {
    role = "WRITER"
    special_group = "projectWriters"
  }

  access {
    role = "OWNER"
    special_group = "projectOwners"
  }

  access {
    role = "OWNER"
    user_by_email = "luana.paris@xertica.com"
  }

  access {
    role = "READER"
    special_group = "projectReaders"
  }
}
resource "google_bigquery_dataset" "gen_ai_transcriptor" {
  dataset_id = "gen_ai_transcriptor"
  project    = "app-springboot-project"
  location   = "us-east1"

  access {
    role = "WRITER"
    special_group = "projectWriters"
  }

  access {
    role = "OWNER"
    special_group = "projectOwners"
  }

  access {
    role = "OWNER"
    user_by_email = "luana.paris@xertica.com"
  }

  access {
    role = "READER"
    special_group = "projectReaders"
  }
}
