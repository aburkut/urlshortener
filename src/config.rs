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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Use a lock to ensure tests don't run in parallel and interfere with each other
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.short_id_length, 7);
        assert_eq!(config.max_collision_attempts, 10);
        assert_eq!(config.cors_allowed_origins, vec!["*"]);
        assert_eq!(config.enable_deduplication, true);
        assert_eq!(config.default_ttl_days, None);
    }

    #[test]
    fn test_from_env_with_defaults() {
        let _lock = ENV_LOCK.lock().unwrap();

        // Clear any existing env vars
        env::remove_var("SHORT_ID_LENGTH");
        env::remove_var("MAX_COLLISION_ATTEMPTS");
        env::remove_var("CORS_ALLOWED_ORIGINS");
        env::remove_var("ENABLE_DEDUPLICATION");
        env::remove_var("DEFAULT_TTL_DAYS");

        let config = AppConfig::from_env();
        assert_eq!(config.short_id_length, 7);
        assert_eq!(config.max_collision_attempts, 10);
        assert_eq!(config.cors_allowed_origins, vec!["*"]);
        assert_eq!(config.enable_deduplication, true);
        assert_eq!(config.default_ttl_days, None);
    }

    #[test]
    fn test_from_env_with_custom_values() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var("SHORT_ID_LENGTH", "10");
        env::set_var("MAX_COLLISION_ATTEMPTS", "5");
        env::set_var("CORS_ALLOWED_ORIGINS", "https://example.com,https://test.com");
        env::set_var("ENABLE_DEDUPLICATION", "false");
        env::set_var("DEFAULT_TTL_DAYS", "30");

        let config = AppConfig::from_env();
        assert_eq!(config.short_id_length, 10);
        assert_eq!(config.max_collision_attempts, 5);
        assert_eq!(
            config.cors_allowed_origins,
            vec!["https://example.com", "https://test.com"]
        );
        assert_eq!(config.enable_deduplication, false);
        assert_eq!(config.default_ttl_days, Some(30));

        // Cleanup
        env::remove_var("SHORT_ID_LENGTH");
        env::remove_var("MAX_COLLISION_ATTEMPTS");
        env::remove_var("CORS_ALLOWED_ORIGINS");
        env::remove_var("ENABLE_DEDUPLICATION");
        env::remove_var("DEFAULT_TTL_DAYS");
    }

    #[test]
    fn test_from_env_with_invalid_values() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var("SHORT_ID_LENGTH", "invalid");
        env::set_var("MAX_COLLISION_ATTEMPTS", "not_a_number");
        env::set_var("ENABLE_DEDUPLICATION", "maybe");

        let config = AppConfig::from_env();
        // Should fall back to defaults
        assert_eq!(config.short_id_length, 7);
        assert_eq!(config.max_collision_attempts, 10);
        assert_eq!(config.enable_deduplication, true);

        // Cleanup
        env::remove_var("SHORT_ID_LENGTH");
        env::remove_var("MAX_COLLISION_ATTEMPTS");
        env::remove_var("ENABLE_DEDUPLICATION");
    }

    #[test]
    fn test_cors_parsing_with_spaces() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var("CORS_ALLOWED_ORIGINS", "  https://a.com  ,  https://b.com  ");

        let config = AppConfig::from_env();
        assert_eq!(
            config.cors_allowed_origins,
            vec!["https://a.com", "https://b.com"]
        );

        env::remove_var("CORS_ALLOWED_ORIGINS");
    }
}
