use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;


#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    identity:i32,
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}


#[derive(Debug,Clone,Serialize, Deserialize)]
struct Profilemark{
    wechat:String,
    nickname:String,
    education:String,
    profession:String,
    email:String
}





#[get("/user/agreement")]
async fn agreement(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select userAgreement from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

#[get("/user/privacy")]
async fn privacy_policy(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select privacyPolicy from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

#[get("/user/aboutus")]
async fn aboutus(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select aboutUs from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}