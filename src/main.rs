use actix_web::{web, App, HttpServer};
mod controller;
use controller as c;


// #[get("/macro-path")]
// async fn index3() -> impl Responder {
// 	HttpResponse::Ok().json("{aaaaa:Heeelll}")
// }
#[actix_rt::main]

async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/",web::get().to(c::insert::index2))
			.route("/ff",web::get().to(c::show::index))
			
	 })	
	   .bind("127.0.0.1:8080")?
	   .run()
	   .await
	 	
}
