use utoipa::OpenApi;

use crate::models::{CreateShortUrlRequest, CreateShortUrlResponse, ErrorResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::shortener::create_short_url,
        crate::routes::shortener::get_full_url,
    ),
    components(
        schemas(CreateShortUrlRequest, CreateShortUrlResponse, ErrorResponse)
    ),
    tags(
        (name = "URL Shortener", description = "URL shortening API endpoints")
    ),
    info(
        title = "URL Shortener API",
        version = "0.1.0",
        description = "A production-ready URL shortening service",
        contact(
            name = "API Support",
            url = "https://github.com/aburkut/urlshortener"
        ),
        license(
            name = "MIT"
        )
    )
)]
pub struct ApiDoc;
