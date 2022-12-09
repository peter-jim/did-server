

use actix_web::cookie::time::OffsetDateTime;
use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use chrono::{Datelike, DateTime, Local, NaiveDate, Utc, NaiveTime, NaiveDateTime, ParseResult};
use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Wechatmark{
    id:String ,   //要查询的微信号id
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct WechatResponse{
    wechat:String    //要查询的微信号id
}



#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserMomentResponse{
    user_id:i32,
    content:String,
    likes:i32,
    img:String,
    // tah:Json<>,
    page_view:i32,
    create_time: DateTime<Utc>
   
}

//返回用户的帖子列表
#[post("/user/moment")]
async fn user_moment( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select user_id, content,likes, img, page_view ,create_time from sys_moment where user_id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< _,UserMomentResponse>(&sql).fetch_all(&pool.pool).await;
    
     
    match res {
        Ok(res) =>{
            let body = serde_json::to_string(&res).unwrap();
   
            // return ;
            HttpResponse::Ok().body(body)
        }
        Err(res) =>{
            HttpResponse::InternalServerError().body("error")
        }
    }
    
}
