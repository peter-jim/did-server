use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[get("/user/agreement")]
async fn agreement(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select userAgreement from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}