use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Serialize, Deserialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct AgreementResponse{
    useragreement:String,
}

#[get("/user/agreement")]
async fn agreement(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select useragreement from sys_plate_config ".to_string();
    let res = sqlx::query_as::< MySql,AgreementResponse>(&sql).fetch_one(&pool.pool).await;
    
     // println!("{:?}",res.unwrap());
     let body = serde_json::to_string(&res.unwrap()).unwrap();
   
     // return ;
     HttpResponse::Ok().body(body)
    
}