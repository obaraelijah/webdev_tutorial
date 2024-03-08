use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub assoc_table: AssocTable,
    pub created: Option<String>,
    pub edited: Option<String>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "assoc_table", rename_all = "snake_case")]
pub enum AssocTable {
    Blog,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagQueryParams {
    pub id: Option<i32>,
    pub table: Option<AssocTable>,
}
