use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Usermark{
    id:u32 ,   //要查询的用户id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserProfileCheckResponse{
    permisson:bool    //要查询的微信号id
}

//判断后台数据是否已存在
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct QuestionNum {
    num: i32,
}

#[post("/user/profilecheck")]
async fn profile_check( user: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!(" select count(userinfo.introduce) as num from sys_user_info userinfo  where userinfo.id = {:?} and (userinfo.introduce = null  or  userinfo.introduce = '') ",user.0.id);
    let res = sqlx::query_as::< _,QuestionNum>(&sql).fetch_one(&pool.pool).await;
    
    // println!("{:?}",res.unwrap());
    if res.is_ok() {

        //不存在
        if res.unwrap().num == 1  {

            let check = false;
            let body = serde_json::to_string(&check).unwrap();
            // return ;
            HttpResponse::Ok().body(body)
            
        }else{
            let check = true;
            let body = serde_json::to_string(&check).unwrap();
            // return ;
            HttpResponse::Ok().body(body)
        }

    } else {
        HttpResponse::InternalServerError().body(" error")
    }
    
   
}