#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String
}