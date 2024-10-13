use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

pub async fn health_check() -> impl Responder {
    let response = HealthStatus {
        status: "alive",
    };
    HttpResponse::Ok().json(response)
}