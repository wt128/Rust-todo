use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db;
pub mod show;
pub mod schema;
pub mod model;

// #[get("/macro-path")]
// async fn index3() -> impl Responder {
// 	HttpResponse::Ok().json("{aaaaa:Heeelll}")
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	db::db_connection();
	HttpServer::new(|| {
		App::new()
			.route("/",web::get().to(show::show))
			//.route("/insert",web::get().to(c::show::index))
			
	 })	
	   .bind("127.0.0.1:8080")?
	   .run()
	   .await
	 	
}
