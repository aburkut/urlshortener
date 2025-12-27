use crate::schema::urls;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Url {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub url: String,
    pub short: String,
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
    pub clicks: i32,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = urls)]
pub struct NewUrl {
    pub url: String,
    pub short: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

/// Request to create a short URL
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateShortUrlRequest {
    /// The long URL to shorten
    #[schema(example = "https://example.com/very/long/url")]
    pub url: String,

    /// Optional TTL in days (time to live)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = 30)]
    pub ttl_days: Option<i64>,
}

/// Response containing the created short URL
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateShortUrlResponse {
    /// The generated short URL identifier
    #[schema(example = "abc1234")]
    pub short_url: String,

    /// Expiration date if TTL was set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::NaiveDateTime>,
}

/// Error response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    #[schema(example = "Invalid URL format")]
    pub error: String,

    /// HTTP status code
    #[schema(example = 400)]
    pub status: u16,
}
