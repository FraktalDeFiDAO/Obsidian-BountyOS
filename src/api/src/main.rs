use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use obsidian_api::{create_schema, AppSchema};
use obsidian_db::Database;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

async fn graphql_handler(
    State(schema): State<Arc<RwLock<AppSchema>>>,
    Json(request): Json<async_graphql::Request>,
) -> impl IntoResponse {
    let schema = schema.read().await;
    let response = schema.execute(request).await;
    Json(response)
}

async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "obsidian-api",
        "version": "0.1.0"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://data/bounties.db".to_string());

    let db = Database::new(&database_url).await?;
    db.init().await?;
    let schema = create_schema(db);
    let state = Arc::new(RwLock::new(schema));

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health_handler))
        .with_state(state);

    let _host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
