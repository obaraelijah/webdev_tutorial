use actix_cors::Cors;
use std::env;

pub fn handle_cors () -> Cors {
    dotenv::dotenv().ok();
    let mut allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

    if cfg!(debug_assertions) {
        allowed_origin = env::var("DEV_FRONTEND_URL").expect("DEV_FRONTEND_URL is not set");
    }

    Cors::permissive()
        .allowed_origin(&allowed_origin)
        .allow_any_method()
        .allow_any_header()
}