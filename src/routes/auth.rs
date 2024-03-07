use crate::db;
use crate::utils::handle_sql_error;
use crate::dtypes::structs::{Auth, Id, Status};
use actix_web::http::{StatusCode, header};
use std::env;
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize)]
struct LoginMessage {
    message: String,
    redirect_url: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    email: Option<String>,
    security_level: Option<i16>,
    status: Option<Status>,
    last_login: Option<String>,
    failed_login_attempts: Option<i32>,
    exp: u64
}

// payload of JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims2 {
    sub: String,
    iss: String,
    aud: String,
    email: Option<String>,
    security_level: Option<i16>,
    exp: u64
}

#[post("/auth/create_user")]
async fn create_user(auth: Json<Auth>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            dotenv::dotenv().ok();

            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            let jwt_hours_active_var = env::var("JWT_HOURS_ACTIVE").expect("JWT_HOURS_ACTIVE is not set");
            let jwt_hours_active: u64 = jwt_hours_active_var.parse().expect("Failed to convert JWT_HOURS_ACTIVE env var to u64");
                    
            // Add (1 hour (3600 seconds) * however many hours) to the current Unix timestamp
            let exp: u64 = since_the_epoch.as_secs() + (3600 * jwt_hours_active as u64);

            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

            let encoding_key = EncodingKey::from_base64_secret(&jwt_secret)
                .unwrap_or_else(|err| {
                eprintln!("Failed to decode base64 secret: {}", err);
                std::process::exit(1);
            });

            let user_claims = Claims2 {
                sub: auth.username.clone(),
                iss: "webservice_tutorial".to_string(),
                aud: "user".to_string(),
                email: auth.email.clone(),
                security_level: Some(10),
                exp,
            };

            let token = encode(&Header::new(Algorithm::HS256), &user_claims, &encoding_key).unwrap();

            let salt = SaltString::generate(&mut OsRng);

            let password_hash_str: String = Argon2::default().hash_password(&auth.password.as_bytes(), &salt).unwrap().to_string();

            let result = sqlx::query_as!(
                Id,
                r#"
                    INSERT INTO auth
                        (
                            email,
                            username,
                            password,
                            security_level
                        )
                    VALUES (
                        $1,
                        $2,
                        $3,
                        $4
                    )
                    RETURNING id
                "#,
                auth.email,
                auth.username,
                password_hash_str,
                auth.security_level
            )
            .fetch_one(&pg)
            .await;

            match result {
                Ok(id) => {
                    HttpResponse::Created()
                        .status(StatusCode::CREATED)
                        .content_type("application/json")
                        .append_header((header::AUTHORIZATION, format!("Bearer {}", token)))
                        .body(
                            serde_json::to_string(&Json(id))
                                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                        )
                },

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[post("/auth/login")]
async fn login(auth: Json<Auth>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let user: Result<Auth, Error> = sqlx::query_as!(
                Auth,
                r#"
                    SELECT
                        id,
                        email,
                        username,
                        password,
                        security_level,
                        status as "status: _",
                        to_char(last_login, 'DD Month YYYY HH12:MI AM') as last_login,
                        failed_login_attempts,
                        to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                        to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                    FROM auth
                    WHERE username = $1
                    LIMIT 1;
                "#,
                &auth.username
            )
            .fetch_one(&pg)
            .await;

            match user {
                Ok(record) => {
                    dotenv::dotenv().ok();

                    let start = SystemTime::now();
                    let since_the_epoch = start.duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");

                    let jwt_hours_active_var = env::var("JWT_HOURS_ACTIVE").expect("JWT_HOURS_ACTIVE is not set");
                    let jwt_hours_active: u64 = jwt_hours_active_var.parse().expect("Failed to convert JWT_HOURS_ACTIVE env var to u64");
                    
                    // Add (1 hour (3600 seconds) * however many hours) to the current Unix timestamp
                    let exp: u64 = since_the_epoch.as_secs() + (3600 * jwt_hours_active as u64);

                    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

                    let encoding_key = EncodingKey::from_base64_secret(&jwt_secret)
                        .unwrap_or_else(|err| {
                            eprintln!("Failed to decode base64 secret: {}", err);
                            std::process::exit(1);
                    });
                    let stored_hash: String = record.password;
                    let parsed_hash = match PasswordHash::new(&stored_hash) {
                        Ok(hash) => hash,
                        Err(_) => {
                            return HttpResponse::InternalServerError().finish();
                        }
                    };

                    let user_claims = Claims {
                        sub: record.username,
                        iss: "webservice_tutorial".to_string(),
                        aud: "user".to_string(),
                        email: record.email,
                        security_level: record.security_level,
                        status: record.status,
                        last_login: record.last_login,
                        failed_login_attempts: record.failed_login_attempts,
                        exp,
                    };

                    let token = encode(&Header::new(Algorithm::HS256), &user_claims, &encoding_key).unwrap();

                    if Argon2::default().verify_password(auth.password.as_bytes(), &parsed_hash).is_ok() {
                        HttpResponse::Ok()
                            .status(StatusCode::OK)
                            .content_type("application/json")
                            .append_header((header::AUTHORIZATION, format!("Bearer {}", token)))
                            .json(LoginMessage {
                                message: "Logged in successfully".to_owned(),
                                redirect_url: "/dashboard".to_owned()
                            })
                    } else {
                        return HttpResponse::Unauthorized().finish();
                    }
                },
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}