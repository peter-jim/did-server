use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Serialize, Deserialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Commentmark{
    id:u32 ,   //要查询评论
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct CommentResponse{
     parent_id:i32,
     user_id:i32,
     content:String,
     to_user_id:i32,
}

#[post("/user/comment")]
async fn user_comment( user: web::Json<Commentmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select  parent_id,user_id,content,to_user_id from sys_comment where moment_id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,CommentResponse>(&sql).fetch_all(&pool.pool).await;

    let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
    
}
