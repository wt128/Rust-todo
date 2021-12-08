use actix_web::{Responder,HttpResponse};

use crate::model::{Todo, New};
use diesel::mysql::MysqlConnection;
use crate::diesel::RunQueryDsl;

pub fn create_todo<'a>(conn: &MysqlConnection,title: &'a str, content: &'a str) {
    use crate::schema::todos;
    let new_todo = New{
        title: title,
        content: content,
    };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .expect("Error this new post");

}

pub async fn insert() -> impl Responder {
    let connection = crate::db::db_connection();
    create_todo(&connection,&"test1",&"aaaaaaaaa");
    HttpResponse::Ok().body(format!("aaaa"))
    

}