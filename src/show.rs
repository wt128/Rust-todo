extern crate diesel;

use crate::db::db_connection;
use crate::model::*;

use diesel::prelude::*;
use actix_web::{Responder};

pub async fn show() -> impl Responder {
     use crate::schema::todos as todo_schema;
     let connection = db_connection();
     let results = todo_schema::dsl::todos
                         .load::<Todo>(&connection)
                         .expect("Error loading");
     
     println!("Displaying {} todo",results.len());
     for todos in results {
          println!("{}",todos.title);
          println!("{}",todos.content);
     }


}