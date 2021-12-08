use crate::schema::todos;
use serde::Serialize;
#[derive(Queryable)]
#[derive(Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct New<'a>{
    pub title: &'a str,
    pub content: &'a str,
}
