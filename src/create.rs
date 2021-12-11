use actix_web::{web,Responder,HttpResponse};

use crate::model::{New,Todo};
use diesel::mysql::MysqlConnection;
use crate::diesel::RunQueryDsl;

pub fn create_todo<'a>(conn: &MysqlConnection,title: &'a str, content: &'a str) -> Todo {
    use crate::schema::todos;
    let new_todo = New{
        title: title,
        content: content,
    };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .expect("Error this new post")
    

}

pub async fn insert(data: web::Json<Todo>) -> impl Responder {
    let connection = crate::db::db_connection();
    let todo = create_todo(&connection,&data.title,&data.content);
    
    HttpResponse::Ok().body(format!("{}",todo.id))

}