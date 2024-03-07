use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod tag;

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::create_blog,
        blog::get_blog_by_id_or_all,
        blog::get_featured_blogs,
        blog::update_blog,
        blog::delete_blog,
    )
}
