use log::{info, error};
use sea_orm::{Database, DatabaseConnection};
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use tokio::net::TcpListener;
use crate::app_state::AppState;
use static_serve::embed_assets;

embed_assets!("admin/dist", compress = true);


pub async fn init_db() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432".to_string());
    let db: DatabaseConnection = Database::connect(&database_url).await?;
    info!("🔄 Running database migrations...");
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await { error!("❌ Failed migrations: {e}"); return Err(e.into()); }
    info!("✅ Database migrations completed successfully!");
    Ok(db)
}
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let db= init_db().await?;
    let port_env = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let requested_port: u16 = port_env.parse().unwrap_or(8080);

    let listener = TcpListener::bind(format!("[::]:{}", requested_port)).await?;
    let actual_port = listener.local_addr()?.port();
    info!("Server starting on port {}", actual_port);

    let state = AppState {  db};
    let app = crate::controllers::routes("/api",state);
    
    let app = app.merge(static_router());
    axum::serve(listener, app).await?;
    Ok(())
}