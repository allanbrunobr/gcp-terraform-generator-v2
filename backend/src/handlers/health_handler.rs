use axum::Json;

pub async fn health_check() -> Json<&'static str> {
    Json("OK")
}
