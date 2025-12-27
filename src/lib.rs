pub mod routes;

mod schema;

pub mod models;

pub mod config;
pub mod errors;
pub mod id_provider;
pub mod openapi;
pub mod repositories;

use rocket_db_pools::Database;

/// Create a Rocket instance for testing
pub fn create_test_rocket() -> rocket::Rocket<rocket::Build> {
    let figment = rocket::Config::figment().merge(("port", 0)); // Use port 0 for testing (OS will assign a random port)

    let config = config::AppConfig::default();

    rocket::custom(figment)
        .mount(
            "/",
            rocket::routes![
                routes::options,
                routes::shortener::create_short_url,
                routes::shortener::get_full_url,
            ],
        )
        .manage(config.clone())
        .attach(routes::Cors::new(config.cors_allowed_origins))
        .attach(routes::DbConn::init())
}
