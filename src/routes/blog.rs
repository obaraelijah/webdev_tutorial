use crate::dtypes::structs::{Blog, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::{get, post, put, delete, web::Query, web::Json, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;


#[post("/blog")]
async fn create_blog(blog: Json<Blog>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                INSERT INTO blog (title, slug, content, image_link, thumbnail_link, featured)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, title, slug, content, image_link, thumbnail_link, featured,
                    to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                    to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                blog.title, 
                blog.slug,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message)
    }
}

#[get("/blog/featured")]
async fn get_featured_blogs() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                Blog,
                r#"
                    SELECT 
                        id,
                        title,
                        slug,
                        content,
                        image_link,
                        thumbnail_link,
                        featured,
                        to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                        to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                    FROM blog
                    WHERE featured = TRUE
                    LIMIT 2;
                "#
            )
            .fetch_all(&pg)
            .await;

            match returned {
                Ok(records) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(records))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),
                Err(e) => handle_sql_error(e),   
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog")]
async fn get_blog_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Blog, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                            to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                        FROM blog
                        WHERE id = $1
                        LIMIT 1;
                    "#,
                    id.id
                )
                .fetch_one(&pg)
                .await;

                match returned {
                    Ok(record) => HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(
                            serde_json::to_string(&Json(record))
                                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                        ),

                    Err(e) => handle_sql_error(e),
                }
            }
            Err(e) => HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(e.message),
        }
    } else {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                            to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                        FROM blog;
                    "#,
                )
                .fetch_all(&pg)
                .await;

                match returned {
                    Ok(record) => HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(
                            serde_json::to_string(&Json(record))
                                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                        ),

                    Err(e) => handle_sql_error(e),
                }
            }
            Err(e) => HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(e.message),
        }
    }
}
