use crate::schema::todos;
#[derive(Queryable)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub content: String,
}