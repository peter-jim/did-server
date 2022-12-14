use actix_web::{
    error, get, post, web, App, Error, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct QuestionResponse {
    questionId: i32,
    discraption: String,
    choice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Questionmark {
    userId: String, //要查询的微信号id
    answer: Vec<ChoiceMark>
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct ChoiceMark{
    questionId: String,
    choice: String,
}

//判断后台数据是否已存在
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct QuestionNum {
    num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct DataResponse {
    questionId: i32,
    discraption: String,
    choice: String,
}




#[get("/user/getquestion")]
async fn get_question(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select questionId,discraption,choice from sys_question ".to_string();
    let res = sqlx::query_as::<MySql, QuestionResponse>(&sql)
        .fetch_all(&pool.pool)
        .await;


    



    let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    HttpResponse::Ok().content_type("application/json").body("[ { \"questionId\":\"1\", \"description  \": \"你希望如何使用这款DAPP\", \"choice\": [ { \"value\":\"A\", \"label\": \"发现开发者\" }, { \"value\":\"B\", \"label\": \"发现开发者\" }, { \"value\":\"C\", \"label\": \"发现开发者\" }, { \"value\":\"D\", \"label\": \"发现开发者\" } ] }, { \"questionId\":\"2\", \"description  \": \"你希望如何使用这款DAPP\", \"choice\": [ { \"value\":\"A\", \"label\": \"发现开发者\" }, { \"value\":\"B\", \"label\": \"发现开发者\" }, { \"value\":\"C\", \"label\": \"发现开发者\" }, { \"value\":\"D\", \"label\": \"发现开发者\" } ] } ]")
}

#[post("/user/postquestion")]
async fn post_question(user: web::Json<Questionmark>, pool: web::Data<AppState>) -> impl Responder {
    println!("接收到信息");




    HttpResponse::Ok().body("success")
}
