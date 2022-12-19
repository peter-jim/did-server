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
    HttpResponse::Ok().content_type("application/json").body("[ { \"questionId\": \"1\", \"description\": \"你希望如何使用这款DAPP\", \"choice\": [ { \"value\": \"A\", \"label\": \"发现开发者\" }, { \"value\": \"B\", \"label\": \"发现好项目\" }, { \"value\": \"C\", \"label\": \"遇见Web3好友\" }, { \"value\": \"D\", \"label\": \"寻找投资人\" } ] }, { \"questionId\": \"2\", \"description  \": \"下面哪个身份适合你\", \"choice\": [ { \"value\": \"A\", \"label\": \"开发者\" }, { \"value\": \"B\", \"label\": \"加密爱好者\" }, { \"value\": \"C\", \"label\": \"学生\" }, { \"value\": \"D\", \"label\": \"社区运营\" } ] }, { \"questionId\": \"3\", \"description\": \"下面哪个公链生态您最感兴趣\", \"choice\": [ { \"value\": \"A\", \"label\": \"Polkadot\" }, { \"value\": \"B\", \"label\": \"Moonbeam\" }, { \"value\": \"C\", \"label\": \"MoonRiver\" }, { \"value\": \"D\", \"label\": \"Ethereum\" } ] } ]")

}
    #[post("/user/postquestion")]
async fn post_question(user: web::Json<Questionmark>, pool: web::Data<AppState>) -> impl Responder {
    println!("接收到信息");

    let user_id = user.0.userId;
    let answer = user.0.answer.clone();
    //依次处理问题
    for i in 0..user.0.answer.len(){

        //问题一
        if answer[i].questionId == "1"{

            let sql = "update sys_user_info set tag1 = ?  where id = ?".to_string();
            let mut tag1 = "Builder";

            match &answer[i].choice as &str  {
                "A" => {
                    tag1 = "找开发"
                },
                "B" => {
                    tag1 = "找项目"
                },
                "C" => {
                    tag1 = "交朋友"
                },
                "D" => {
                    tag1 = "找投资"
                },
                _=>()
            }
            let _update_identity = sqlx::query::<MySql>(&sql)
            .bind(tag1)
            .bind(user_id.clone())
            .execute(&pool.pool)
            .await;
            
        }else if answer[i].questionId == "2" {
            let sql = "update sys_user_info set identity = ?  where id = ?".to_string();
            let mut identity = "Web3er";
            match &answer[i].choice as &str  {
                "A" => {
                    identity = "Coder";
                    // println!("{:?}",identity);
                
                },
                "B" => {
                    identity = "Web3er";
                },
                "C" => {
                    identity = "Student";
                },
                "D" => {
                    identity = "KOL";
                },
                _=>()
            }
            let _update_identity = sqlx::query::<MySql>(&sql)
            .bind(identity)
            .bind(user_id.clone())
            .execute(&pool.pool)
            .await;
        }else if answer[i].questionId == "3" {
            let sql = "update sys_user_info set tag2 = ?  where id = ?".to_string();
            let mut tag2 = "Builder";

            match &answer[i].choice as &str  {
                "A" => {
                    tag2 = "Polkadot"
                },
                "B" => {
                    tag2 = "Moonbeam"
                },
                "C" => {
                    tag2 = "MoonRiver"
                },
                "D" => {
                    tag2 = "Ethereum"
                },
                _=>()
            }
            let _update_identity = sqlx::query::<MySql>(&sql)
            .bind(tag2)
            .bind(user_id.clone())
            .execute(&pool.pool)
            .await;
        }



    }
    HttpResponse::Ok().body("success")
}
