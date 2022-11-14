

use actix_web::{get, web, App, HttpServer, Responder};
use did_server::{AppState, user_moment::user_moment, user_info::user_info, user_wechat::user_wechat, order_did::order_did, privacy_policy::privacy_policy, agreement::agreement,didrecommand::didrecommand,user_comment::user_comment, aboutus::about_us};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};


#[get("/hello/{name}
")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}




#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    print!("connecting mysql ");
    let pool = MySqlPoolOptions::new()
    .max_connections(50)
    .connect("mysql://test:123456@114.55.67.80:3306/social")
    .await.unwrap_or_else(|_|
         { std::process::exit(0x0100) });

    print!("服务启动");

    HttpServer::new(move|| {
        App::new().app_data( web::Data::new( AppState{pool:pool.clone() } ))   //公共数据传入一个结构体，方便操作
        .service(greet)
        .service(didrecommand)
        .service(agreement)
        .service(privacy_policy)
        .service(about_us)
        .service(order_did)
        .service(user_wechat)
        .service(user_info)
        .service(user_moment)
        .service(user_comment)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
