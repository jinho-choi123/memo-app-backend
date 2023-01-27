mod routers;
mod db;
mod utils;
use actix_web::{App,HttpServer};
use routers::checkserver::checkserver;
use utils::check_dotenv::check_dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    check_dotenv();
    println!("creating httpserver at port {}", 8080);
    HttpServer::new(|| {
        App::new()
            .service(checkserver)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}