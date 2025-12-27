use crate::schema::urls;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

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
#[diesel(table_name=urls)]
pub struct NewUrl {
    pub url: String,
    pub short: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateShortUrlRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_days: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateShortUrlResponse {
    pub short_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::NaiveDateTime>,
}