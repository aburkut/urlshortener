use rocket::fairing::{Info, Fairing, Kind};
use rocket::{Request, Response};
use rocket::http::Header;
use rocket_db_pools::Database;

pub mod shortener;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Just to add CORS headers via the fairing
}

pub struct Cors {
    allowed_origins: Vec<String>,
}

impl Cors {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self { allowed_origins }
    }
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        let origin: &'static str = if self.allowed_origins.contains(&"*".to_string()) {
            "*"
        } else {
            // For production, you'd want to check the request origin against allowed_origins
            "*"
        };

        res.set_header(Header::new("Access-Control-Allow-Origin", origin));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        // Note: Access-Control-Allow-Credentials cannot be used with Access-Control-Allow-Origin: *
        // If you need credentials, specify a specific origin instead of "*"
    }
}