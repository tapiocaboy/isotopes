use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::retry::retry;
use std::time::Duration;
use chrono::Utc;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiModuleUpdate {
    pub ai_id: i32,
    pub business_id: String,
    pub module_id: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiModule {
    pub ai_id: i32,
    pub business_id: String,
    pub module_id: String,
    pub created: chrono::DateTime<Utc>,
    pub updated: chrono::DateTime<Utc>,
    pub active: bool,
}

/// Subscribe to AI module updates
/// # Arguments
/// * `pool` - The database connection pool
/// * `payload` - The AI module update payload
/// # Returns
/// * `HttpResponse` - The HTTP response
pub async fn subscribe_ai_module_updates(
    pool: web::Data<PgPool>,
    payload: web::Json<AiModuleUpdate>,
) -> impl Responder {
    let result = retry(
        || {
            let pool = pool.clone();
            let payload = payload.clone();
            Box::pin(async move {
                update_ai_module(&pool, &payload).await
            })
        },
        3,
        Duration::from_secs(1),
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "AI module updated"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": e.to_string()})),
    }
}

/// update AI module
/// # Arguments
/// * `pool` - The database connection pool
/// * `update` - The AI module update payload
/// # Returns
/// * `Result<(), sqlx::Error>` - The result of the operation
/// # Errors
/// * `sqlx::Error` - The error type
async fn update_ai_module(pool: &PgPool, update: &AiModuleUpdate) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE ai_modules
        SET business_id = $1, module_id = $2, active = $3, updated = CURRENT_TIMESTAMP
        WHERE ai_id = $4
        "#,
        update.business_id,
        update.module_id,
        update.active,
        update.ai_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get AI module
/// # Arguments
/// * `pool` - The database connection pool
/// * `ai_id` - The AI module ID
/// # Returns
/// * `HttpResponse` - The HTTP response
/// # Errors
/// * `sqlx::Error` - The error type
/// * `actix_web::Error` - The error type
/// * `serde_json::Error` - The error type
/// * `chrono::ParseError` - The error type
/// * `std::io::Error` - The error type
/// * `std::time::SystemTimeError` - The error type
pub async fn get_ai_module(
    pool: web::Data<PgPool>,
    ai_id: web::Path<i32>,
) -> impl Responder {
    let result = fetch_ai_module(&pool, ai_id.into_inner()).await;
    match result {
        Ok(Some(ai_module)) => HttpResponse::Ok().json(ai_module),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({"status": "error", "message": "AI module not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": e.to_string()})),
    }
}

pub async fn get_all_ai_modules(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = fetch_all_ai_modules(&pool).await;
    match result {
        Ok(ai_modules) => HttpResponse::Ok().json(ai_modules),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": e.to_string()})),
    }
}

async fn fetch_ai_module(pool: &PgPool, ai_id: i32) -> Result<Option<AiModule>, sqlx::Error> {
    sqlx::query_as!(
        AiModule,
        r#"
        SELECT ai_id, business_id, module_id, created, updated, active
        FROM ai_modules
        WHERE ai_id = $1
        "#,
        ai_id
    )
    .fetch_optional(pool)
    .await
}

async fn fetch_all_ai_modules(pool: &PgPool) -> Result<Vec<AiModule>, sqlx::Error> {
    sqlx::query_as!(
        AiModule,
        r#"
        SELECT ai_id, business_id, module_id, created, updated, active
        FROM ai_modules
        "#
    )
    .fetch_all(pool)
    .await
}
