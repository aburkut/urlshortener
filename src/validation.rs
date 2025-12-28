use crate::errors::AppError;

/// Validates if a URL is properly formatted
pub fn validate_url(url_str: &str) -> Result<(), AppError> {
    url::Url::parse(url_str).map_err(|e| AppError::InvalidUrl(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_https_url() {
        assert!(validate_url("https://example.com").is_ok());
    }

    #[test]
    fn test_valid_http_url() {
        assert!(validate_url("http://example.com").is_ok());
    }

    #[test]
    fn test_valid_url_with_path() {
        assert!(validate_url("https://example.com/path/to/resource").is_ok());
    }

    #[test]
    fn test_valid_url_with_query() {
        assert!(validate_url("https://example.com?param=value&other=123").is_ok());
    }

    #[test]
    fn test_valid_url_with_fragment() {
        assert!(validate_url("https://example.com/page#section").is_ok());
    }

    #[test]
    fn test_valid_url_with_port() {
        assert!(validate_url("https://example.com:8080/api").is_ok());
    }

    #[test]
    fn test_valid_url_with_subdomain() {
        assert!(validate_url("https://api.example.com").is_ok());
    }

    #[test]
    fn test_invalid_url_no_scheme() {
        let result = validate_url("example.com");
        assert!(result.is_err());
        if let Err(AppError::InvalidUrl(msg)) = result {
            assert!(msg.contains("relative URL without a base"));
        }
    }

    #[test]
    fn test_invalid_url_empty() {
        let result = validate_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_url_spaces() {
        let result = validate_url("not a url");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_url_special_chars() {
        let result = validate_url("http://exam ple.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_localhost() {
        assert!(validate_url("http://localhost:3000").is_ok());
    }

    #[test]
    fn test_valid_ip_address() {
        assert!(validate_url("http://192.168.1.1").is_ok());
    }
}
