extern crate diesel;

use crate::db::db_connection;
use crate::model::*;
//use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use actix_web::{Responder,HttpResponse};
// use std::any::type_name;
// use serde::Serialize;

pub async fn show() -> impl Responder {
     use crate::schema::todos as todo_schema;
     let connection = db_connection();
     
     let results = todo_schema::dsl::todos
                         .load::<Todo>(&connection)
                         .expect("Error loading");
     
     println!("Displaying {} todo",results.len());
     for todos in &results {
          println!("{}",todos.title);
          println!("{}",todos.content);
     }
     
     
     HttpResponse::Ok().json(&results)


}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// 問題 きそんの構造体変数をどうシリアライズすればいいか。