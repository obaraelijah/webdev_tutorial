use actix_web::{web, App, HttpServer};

pub mod dtypes;
pub mod db;
pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .service(routes::blog())
                .service(routes::tag())
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    .expect("ERROR: src/main.rs: server initialization fail");

    Ok(())
}