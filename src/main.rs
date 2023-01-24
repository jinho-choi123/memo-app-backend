mod routers;
use actix_web::{App,HttpServer};
use routers::checkserver::checkserver;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(checkserver)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}