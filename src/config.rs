use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub short_id_length: usize,
    pub max_collision_attempts: usize,
    pub cors_allowed_origins: Vec<String>,
    pub enable_deduplication: bool,
    pub default_ttl_days: Option<i64>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            short_id_length: 7,
            max_collision_attempts: 10,
            cors_allowed_origins: vec!["*".to_string()],
            enable_deduplication: true,
            default_ttl_days: None, // No expiration by default
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let short_id_length = env::var("SHORT_ID_LENGTH")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7);

        let max_collision_attempts = env::var("MAX_COLLISION_ATTEMPTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .ok()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|| vec!["*".to_string()]);

        let enable_deduplication = env::var("ENABLE_DEDUPLICATION")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(true);

        let default_ttl_days = env::var("DEFAULT_TTL_DAYS")
            .ok()
            .and_then(|s| s.parse().ok());

        Self {
            short_id_length,
            max_collision_attempts,
            cors_allowed_origins,
            enable_deduplication,
            default_ttl_days,
        }
    }
}
