

use std::fs::File;

use actix_web::{get, web, App, HttpServer, Responder};
use did_server::{AppState, user_moment::user_moment, user_info::{user_info, user_profile, change_visible, update_profile}, user_wechat::user_wechat, order_did::order_did, privacy_policy::privacy_policy, agreement::agreement,didrecommand::didrecommand,user_comment::user_comment, aboutus::about_us, user_question::{get_question, post_question, check_question, gap_question, post_gapquestion}, user_frzs::{add_friend, user_friendslist}, publish_check::publish_check, user_frzsnew::{user_frzsnews, user_profrzsnews, user_notification}, user_tag::user_tag, user_report::user_report, profile_check::profile_check
};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!" )
}




#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    /////从input.json文件读取输入数据
    let f = File::open("src/config/conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let mysql = v["mysql"].as_str().unwrap();

    println!("connecting mysql {:?} !",mysql);
    let pool = MySqlPoolOptions::new()
    .max_connections(50)
    .connect(mysql)
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
        .service(get_question)
        .service(post_question)
        .service(add_friend)
        .service(publish_check)
        .service(user_profrzsnews)
        .service(user_frzsnews)
        .service(user_tag)
        .service(check_question)
        .service(gap_question)
        .service(post_gapquestion)
        .service(user_notification)
        .service(user_report)
        .service(user_friendslist)
        .service(user_profile)
        .service(change_visible)
        .service(profile_check)
        .service(update_profile)
        
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
