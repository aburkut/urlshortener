use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("URL not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Failed to generate unique short URL after {0} attempts")]
    TooManyCollisions(usize),

    #[error("URL has expired")]
    UrlExpired,
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            AppError::InvalidUrl(msg) => (Status::BadRequest, format!("Invalid URL: {}", msg)),
            AppError::NotFound => (Status::NotFound, "Short URL not found".to_string()),
            AppError::DatabaseError(_) => (
                Status::InternalServerError,
                "Database error occurred".to_string(),
            ),
            AppError::TooManyCollisions(attempts) => (
                Status::InternalServerError,
                format!(
                    "Failed to generate unique short URL after {} attempts",
                    attempts
                ),
            ),
            AppError::UrlExpired => (Status::Gone, "This short URL has expired".to_string()),
        };

        let json = json!({
            "error": message,
            "status": status.code
        });

        Response::build()
            .status(status)
            .sized_body(None, std::io::Cursor::new(json.to_string()))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}
