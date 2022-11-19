//获取正在发行的did
use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct DidSellListResponse{
     id:i32,
     price:i32,
     title:String,
     discraption:String,
     circulation:i32 //已发行量
}



#[get("/order/did")]
async fn order_did(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select id, price,circulation,price,title,discraption from sys_did  ".to_string();
    let res = sqlx::query_as::< MySql,DidSellListResponse>(&sql).fetch_all(&pool.pool).await;
    let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
    
}