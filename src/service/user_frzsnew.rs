use actix_web::cookie::time::OffsetDateTime;
use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use chrono::{Datelike, DateTime, Local, NaiveDate, Utc, NaiveTime, NaiveDateTime, ParseResult};
use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Usermark{
    userid:String ,   //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserProcessmark{
    id:String ,   //id
    frizesid:String ,   //要查询的微信号id
    result:String,
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct NewsResponse{
    id:i32,
    friendsid:i32,
    create_time:DateTime<Utc>,
    content:String,
    head_sculpture:String
}

#[get("/user/frzsnews")]
async fn user_frzsnews( user: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    //state = 0 未处理， 1 通过，2拒绝。
    let sql = format!("select id, friendsid,create_time,content,head_sculpture from sys_user_friends where userid = {:?} and state = 0 ",user.0.userid.parse::<i32>().unwrap());
    println!("{:?}",sql.clone());
    let res = sqlx::query_as::< _,NewsResponse>(&sql).fetch_all(&pool.pool).await;
    


    match res {
        Ok(res) => {
            let body = serde_json::to_string(&res).unwrap();
   
            // return ;

            HttpResponse::Ok().body(body)
            
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("no info")
        }


    }
}

#[post("/user/profrzsnews")]
async fn user_profrzsnews( user: web::Json<UserProcessmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    //state = 0 未处理， 1 通过，2拒绝。
    let state = user.result.clone();
    
    let mut sql: String = "".to_string();
    if state == "1" {
        sql = "update sys_user_friends set state = 1 where id = ?".to_string();
    }
    
    if state == "2" {
        sql = "update sys_user_friends set state = 2 where id = ?".to_string();
    }else{
        HttpResponse::InternalServerError().body("error");
    }
    
   

    let insert_res = sqlx::query::<MySql>(&sql)
        .bind(user.0.id)
        .execute(&pool.pool)
        .await;
    match insert_res {
        Ok(res) => {
            HttpResponse::Ok().body("success")
            
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("error")
        }
    }


}
