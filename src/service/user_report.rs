use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Momentmark{
    momentId:i32 ,   //要momentid
    userId:i32
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct MomentReportResponse{
    id:String    //要查询的微信号id
}


#[post("/user/report")]
async fn user_report( user: web::Json<Momentmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    HttpResponse::Ok().body("success")
}