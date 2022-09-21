
use actix_web::{get, web, App, HttpServer, Responder, post};
use serde::{Serialize, Deserialize};





#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
        .service(didrecommand)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



