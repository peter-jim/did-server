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
    userId:i32 ,   //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserProcessmark{
    id:i32 ,   //id
    friendsid :i32 ,   //要查询的微信号id
    result:i32,
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct NewsSQL{
    id:i32,
    nickname:String,
    friendsid:i32,
    createTime:DateTime<Utc>,
    content:String,
    headSculpture:String,
    // tag: Vec<String>

}
#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct NewsResponse{
    id:i32,
    nickname:String,
    friendsid:i32,
    createTime:DateTime<Utc>,
    content:String,
    headSculpture:String,
    tag: Vec<String>
}


#[post("/user/notification")]
async fn user_notification( user: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select wechat from sys_user_info where id = {:?} ",user.0.userId);
    println!("{:?}",sql.clone());
    let check = true;
    let body = serde_json::to_string(&check).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
}


#[post("/user/frzsnews")]
async fn user_frzsnews( user: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    //state = 0 未处理， 1 通过，2拒绝。
    let sql = format!("select id,nickname, friendsid,createTime,content,headSculpture from sys_user_friends where userId = {:?} and state = 0 ",user.0.userId);
    println!("{:?}",sql.clone());
    let res = sqlx::query_as::< _,NewsSQL>(&sql).fetch_all(&pool.pool).await;
    


    match res {
        Ok(res) => {
            
            let mut vec = Vec::new();
            vec.push("CEO".to_string());
            vec.push("Builder".to_string());
            vec.push("Builder".to_string());

            let user_info = res.clone();
            let mut user_vec: Vec<NewsResponse> = Vec::new();

            for i in 0..res.len(){
                let s = res[i].clone();

                let news = NewsResponse{
                    id:s.id,
                    nickname:s.nickname,
                    friendsid:s.friendsid,
                    createTime:s.createTime,
                    content:s.content,
                    headSculpture:s.headSculpture,
                    tag: vec.clone()
                };

                user_vec.push(news);

            }
            

            let body = serde_json::to_string(&user_vec).unwrap();
            HttpResponse::Ok().content_type("application/json").body(body)
            
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
    if state == 1 {
        sql = "update sys_user_friends set state = 1 where id = ?".to_string();
    }
    
    if state == 2 {
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
            HttpResponse::Ok().content_type("application/json") .body("success")
            
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("error")
        }
    }


}

