use actix_web::{
    error, get, post, web, App, Error, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct AddFriendMark {
    userid: i32, //发送的微信号用户id
    friendid: i32,
    publickey:String,
    cipher: String, //密文内容
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct AddFriendResponse {
    result: String,
}

//判断后台数据是否已存在
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct QuestionNum {
    num: i32,
}

#[post("/user/add/friend")]
async fn add_friend(user: web::Json<AddFriendMark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let check_sql =
        " select count(id) as num  from sys_user_friends where userid = ? and  friendsid = ?  ";

    let res = sqlx::query_as::<MySql, QuestionNum>(check_sql)
        .bind(user.0.userid)
        .bind(user.0.friendid)
        .fetch_one(&pool.pool)
        .await;

    if res.is_ok() {

        //不存在
        if res.unwrap().num < 1  {

             //不存在执行插入
             let sql = r#"insert into  sys_user_friend (userid,friendsid,publickey,create_time,state,content)
             VALUES (?,?,?,?,?,?)
             "#;
             let insert_res = sqlx::query::<MySql>(sql)
                 .bind(user.0.userid)
                 .bind(user.0.friendid)
                 .bind(user.0.publickey)
                 .bind( get_local_time() )
                 .bind(0)
                 .bind(user.0.cipher)
                 .execute(&pool.pool)
                 .await;
 
             match insert_res {
                 Ok(result) => {
                     println!("{:?}", result);
                 }
                 Err(err) => {
                     println!("{:?}", err);
                 }
             };
            
        }
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::InternalServerError().body("add friend error")
    }
}





fn get_local_time() -> String{

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    now.to_string()

}