use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod tag;
pub mod auth;

pub fn auth() -> impl HttpServiceFactory {
    (
        auth::login, 
        auth::create_user
    )
}

pub fn tag() -> impl HttpServiceFactory {
    (
        tag::get_tag_by_id_or_all,
        tag::delete_tag,
        tag::create_tag,
        tag::update_tag,
    )
}

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::create_blog,
        blog::get_blog_by_id_or_all,
        blog::get_featured_blogs,
        blog::update_blog,
        blog::delete_blog,
    )
}
