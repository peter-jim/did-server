use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Usermark{
    id:String ,   //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct PublishCheckResponse{
    permisson:bool    //要查询的微信号id
}


#[post("/user/publishcheck")]
async fn publish_check( user: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    // let sql = format!("select wechat from sys_user_info where id = {:?} ",user.0.id);
    // println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,WechatResponse>(&sql).fetch_one(&pool.pool).await;
    
    // res[]

    // for i in res{
    //     println!("{:?}",i."row");
    // }
    // format!("{:?}", serde_json::to_value(&res.unwrap())  )
    let check = true;


    let body = serde_json::to_string(&check).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
}