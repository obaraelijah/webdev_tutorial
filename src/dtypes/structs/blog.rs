use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub featured: bool,
    pub created: Option<String>,
    pub edited: Option<String>,
}
