use crate::dtypes::structs::{ Id, Tag, AssocTable, TagQueryParams};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::{get, put, post, delete, web::Query, HttpResponse, web::Json};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/tag")]
async fn create_tag(tag: Json<Tag>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Tag, Error> = sqlx::query_as!(
                Tag,
                r#"
                    INSERT INTO tag (name, assoc_table)
                    VALUES ($1, $2)
                    RETURNING
                        id,
                        name,
                        assoc_table as "assoc_table: AssocTable",
                        to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                        to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                tag.name,
                tag.assoc_table as AssocTable,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e))
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

#[get("/tag")]
async fn get_tag_by_id_or_all(Query(params): Query<TagQueryParams>) -> HttpResponse {
    match params.id {
        Some(id) => {
            match params.table {
                Some(table) => {
                    match db::connect().await {
                        Ok(pg) => {
                            let returned: Result<Tag, Error> = sqlx::query_as!(
                        Tag,
                        r#"
                            SELECT
                                id,
                                name,
                                assoc_table as "assoc_table: AssocTable",
                                to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                                to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                            FROM tag
                            WHERE id = $1 AND
                            assoc_table = $2
                            LIMIT 1;
                        "#,
                        id,
                        table as AssocTable
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
                },
                None => {
                    match db::connect().await {
                        Ok(pg) => {
                            let returned: Result<Tag, Error> = sqlx::query_as!(
                            Tag,
                            r#"
                                SELECT
                                    id,
                                    name,
                                    assoc_table as "assoc_table: AssocTable",
                                    to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                                    to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                                FROM tag
                                WHERE id = $1
                                LIMIT 1;
                            "#,
                            id
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
                }
            }
        },
        None => {
            match params.table {
                Some(table) => {
                    match db::connect().await {
                        Ok(pg) => {
                            let returned: Result<Vec<Tag>, Error> = sqlx::query_as!(
                    Tag,
                    r#"
                        SELECT
                            id,
                            name,
                            assoc_table as "assoc_table: AssocTable",
                            to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                            to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                        FROM tag
                        WHERE assoc_table = $1
                    "#,
                        table as AssocTable
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
                },
                None => {
                    match db::connect().await {
                        Ok(pg) => {
                            let returned: Result<Vec<Tag>, Error> = sqlx::query_as!(
                                Tag,
                                r#"
                                    SELECT
                                        id,
                                        name,
                                        assoc_table as "assoc_table: AssocTable",
                                        to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                                        to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                                    FROM tag
                                "#
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
        }
    }
}

#[put("/blog")]
async fn update_slug(tag: Json<Tag>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Tag, Error> = sqlx::query_as!(
                Tag,
                r#"
                    INSERT INTO tag (id, name, assoc_table)
                    VALUES ($1, $2, $3)
                    ON_CONFLICT (id)
                    DO UPDATE SET
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        assoc_table = EXCLUDED.assoc_table,
                        edited = now()
                    RETURNING
                        id,
                        name,
                        assoc_table as "assoc_table: AssocTable",
                        to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                        to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                tag.id,
                tag.name,
                tag.assoc_table as AssocTable,
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
}

#[delete("/tag")]
async fn delete_tag(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =    
                sqlx::query_as!(Tag, "DELETE FROM Tag WHERE id = $1;", id.id)
                    .execute(&pg)
                    .await;
            match returned {
                Ok(_) => HttpResponse::NoContent()
                    .status(StatusCode::NO_CONTENT)
                    .content_type("application/json")
                    .finish(),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message)
        }
}