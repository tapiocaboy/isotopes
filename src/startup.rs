use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;


use sqlx::PgPool;
use std::net::TcpListener;
use crate::subscription_service;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscribe/ai_module", web::post().to(subscription_service::subscribe_ai_module_updates))
            .route("/ai_modules/{id}", web::get().to(subscription_service::get_ai_module))
            .route("/ai_modules", web::get().to(subscription_service::get_all_ai_modules))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}