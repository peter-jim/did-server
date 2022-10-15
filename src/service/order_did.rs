//获取正在发行的did
use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;



#[get("/order/did")]
async fn order_did(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select circulation,price,title,discraption from sys_did where status = 1 ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}