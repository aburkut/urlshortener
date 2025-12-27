use chrono::{Duration, Utc};
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{get, post, State};
use rocket_db_pools::Connection;
use tracing::{error, info, warn};

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::id_provider::{IDProvider, NanoIDProvider};
use crate::models::{CreateShortUrlRequest, CreateShortUrlResponse, NewUrl};
use crate::repositories::UrlRepository;
use crate::routes::DbConn;

/// Validates if a URL is properly formatted
fn validate_url(url_str: &str) -> Result<(), AppError> {
    url::Url::parse(url_str).map_err(|e| AppError::InvalidUrl(e.to_string()))?;
    Ok(())
}

/// Create a short URL
///
/// Creates a shortened URL from a long URL. Optionally accepts a TTL (time to live) in days.
/// If the same URL is submitted multiple times with deduplication enabled, the same short URL will be returned.
#[utoipa::path(
    post,
    path = "/create_short_url",
    request_body = CreateShortUrlRequest,
    responses(
        (status = 201, description = "Short URL created successfully", body = CreateShortUrlResponse),
        (status = 400, description = "Invalid URL format", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "URL Shortener"
)]
#[post("/create_short_url", format = "json", data = "<request>")]
pub async fn create_short_url(
    mut db: Connection<DbConn>,
    config: &State<AppConfig>,
    request: Json<CreateShortUrlRequest>,
) -> Result<Created<Json<CreateShortUrlResponse>>, AppError> {
    // Validate URL format
    validate_url(&request.url)?;
    info!("Creating short URL for: {}", request.url);

    // Check if URL already exists (deduplication)
    if config.enable_deduplication {
        match UrlRepository::find_by_url(&mut db, &request.url).await {
            Ok(existing_url) => {
                info!(
                    "Found existing short URL for this URL: {}",
                    existing_url.short
                );
                return Ok(Created::new("/").body(Json(CreateShortUrlResponse {
                    short_url: existing_url.short,
                    expires_at: existing_url.expires_at,
                })));
            }
            Err(_) => {
                // No existing URL found, continue with creation
            }
        }
    }

    // Calculate expiration date
    let expires_at = request
        .ttl_days
        .or(config.default_ttl_days)
        .map(|days| (Utc::now() + Duration::days(days)).naive_utc());

    // Generate short URL using ID provider with collision detection
    let id_provider = NanoIDProvider::new(config.short_id_length);
    let mut short_url = id_provider.provide();
    let mut attempts = 0;

    loop {
        attempts += 1;

        if attempts > config.max_collision_attempts {
            error!(
                "Failed to generate unique short URL after {} attempts",
                attempts
            );
            return Err(AppError::TooManyCollisions(attempts));
        }

        match UrlRepository::find_by_short(&mut db, &short_url).await {
            Ok(_) => {
                // Collision detected, generate new one
                warn!(
                    "Collision detected for short URL: {} (attempt {})",
                    short_url, attempts
                );
                short_url = id_provider.provide();
            }
            Err(diesel::result::Error::NotFound) => {
                // Good, this short URL is available
                break;
            }
            Err(e) => {
                error!("Database error while checking for collision: {}", e);
                return Err(AppError::DatabaseError(e.to_string()));
            }
        }
    }

    // Create new URL entry
    let new_url = NewUrl {
        url: request.url.clone(),
        short: short_url.clone(),
        expires_at,
    };

    match UrlRepository::create(&mut db, new_url).await {
        Ok(url) => {
            info!(
                "Successfully created short URL: {} -> {}",
                short_url, request.url
            );
            Ok(Created::new("/").body(Json(CreateShortUrlResponse {
                short_url,
                expires_at: url.expires_at,
            })))
        }
        Err(e) => {
            error!("Failed to create URL in database: {}", e);
            Err(AppError::DatabaseError(e.to_string()))
        }
    }
}

/// Redirect to the original URL
///
/// Takes a short URL identifier and redirects to the original long URL.
/// Increments the click counter and checks if the URL has expired.
#[utoipa::path(
    get,
    path = "/{short_url}",
    params(
        ("short_url" = String, Path, description = "Short URL identifier", example = "abc1234")
    ),
    responses(
        (status = 303, description = "Redirect to original URL"),
        (status = 404, description = "Short URL not found", body = ErrorResponse),
        (status = 410, description = "Short URL has expired", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "URL Shortener"
)]
#[get("/<short_url>")]
pub async fn get_full_url(
    mut db: Connection<DbConn>,
    short_url: String,
) -> Result<rocket::response::Redirect, AppError> {
    match UrlRepository::find_by_short(&mut db, &short_url).await {
        Ok(url) => {
            // Check if URL has expired
            if let Some(expires_at) = url.expires_at {
                if expires_at < Utc::now().naive_utc() {
                    warn!("Attempted to access expired short URL: {}", short_url);
                    return Err(AppError::UrlExpired);
                }
            }

            // Increment click counter
            if let Err(e) = UrlRepository::increment_clicks(&mut db, url.id).await {
                warn!("Failed to increment click counter for {}: {}", short_url, e);
                // Don't fail the request, just log the error
            }

            info!(
                "Redirecting {} -> {} (clicks: {})",
                short_url,
                url.url,
                url.clicks + 1
            );
            Ok(rocket::response::Redirect::to(url.url))
        }
        Err(diesel::result::Error::NotFound) => {
            warn!("Short URL not found: {}", short_url);
            Err(AppError::NotFound)
        }
        Err(e) => {
            error!(
                "Database error while retrieving short URL {}: {}",
                short_url, e
            );
            Err(AppError::DatabaseError(e.to_string()))
        }
    }
}
