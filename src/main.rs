use actix_web::{web, App, HttpServer};

pub mod db;
pub mod dtypes;
pub mod middleware;
pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().wrap(middleware::handle_cors()).service(
            web::scope("/api/v1")
                .wrap(middleware::JWTAuth)
                .wrap(middleware::CaptureUri)
                .service(routes::auth())
                .service(routes::blog())
                .service(routes::tag()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("ERROR: src/main.rs: server initialization fail");

    Ok(())
}
