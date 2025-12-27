extern crate urlshortener;

use rocket::{Error, Ignite, Rocket};
use rocket_db_pools::Database;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use urlshortener::config::AppConfig;
use urlshortener::routes::DbConn;

pub struct Server {
    port: u16,
    config: AppConfig,
}

impl Server {
    pub fn new(port: u16, config: AppConfig) -> Self {
        Server { port, config }
    }

    async fn run(self) -> Result<Rocket<Ignite>, Error> {
        let figment = rocket::Config::figment()
            .merge(("port", self.port))
            .merge(("cli_colors", false)); // Disable colored output

        rocket::custom(figment)
            .mount(
                "/",
                rocket::routes![
                    urlshortener::routes::options,
                    urlshortener::routes::shortener::create_short_url,
                    urlshortener::routes::shortener::get_full_url,
                ],
            )
            .manage(self.config.clone())
            .attach(urlshortener::routes::Cors::new(
                self.config.cors_allowed_origins,
            ))
            .attach(DbConn::init())
            .launch()
            .await
    }
}

#[rocket::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false) // Disable ANSI color codes
                .with_target(false), // Hide module paths
        )
        .init();

    // Load configuration from environment variables
    let config = AppConfig::from_env();
    tracing::info!("Configuration loaded: {:?}", config);

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    tracing::info!("Starting URL Shortener server on port {}", port);

    let server = Server::new(port, config);
    if let Err(e) = server.run().await {
        tracing::error!("Server failed to start: {}", e);
    }
}
