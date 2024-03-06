use actix_web::http::StatusCode;
use actix_web::{web::Json, HttpResponse};
use sqlx::Error;

pub fn handle_sql_error(e: Error) -> HttpResponse {
    let status_code = match &e {
        Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::RowNotFound => StatusCode::NOT_FOUND,
        Error::Decode(_) | Error::Io(_) | Error::Migrate(_) | Error::Tls(_) | Error::Protocol(_) | Error::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::PoolTimedOut => StatusCode::GATEWAY_TIMEOUT,
        Error::PoolClosed => StatusCode::SERVICE_UNAVAILABLE,
        Error::WorkerCrashed => StatusCode::INTERNAL_SERVER_ERROR,
        Error::ColumnNotFound(_) | Error::ColumnDecode { .. } | Error::TypeNotFound { .. } => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let error_message = match &e {
        Error::Database(err) => format!("Database error: {}", err),
        Error::RowNotFound => "Row not found".to_string(),
        Error::Decode(err) => format!("Decode error: {}", err),
        Error::Io(err) => format!("IO error: {}", err),
        Error::Migrate(err) => format!("Migrate error: {}", err),
        Error::PoolTimedOut => "Connection pool timed out".to_string(),
        Error::PoolClosed => "Connection pool is closed".to_string(),
        Error::WorkerCrashed => "Worker has crashed".to_string(),
        Error::Tls(err) => format!("TLS error: {}", err),
        Error::Protocol(err) => format!("Protocol error: {}", err),
        Error::ColumnNotFound(column) => format!("Column not found: {}", column),
        Error::ColumnDecode { index, source } => format!("Error decoding column at index {}: {}", index, source),
        Error::Configuration(err) => format!("Configuration error: {}", err),
        Error::TypeNotFound { type_name } => format!("Type not found: {}", type_name),
        _ => "Non-exhaustive error variant encountered".to_string(),
    };

    HttpResponse::build(status_code)
        .content_type("application/json")
        .body(serde_json::to_string(&Json(error_message)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
}
