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
    userId: u32, //要查询的微信号id
    discraption: String,
    questionId: u32,
    updateTime: String,
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
    HttpResponse::Ok().body("[ { \"questionId\":\"1\", \"discraption\": \"你希望如何使用这款DAPP\", \"choice\": [ { \"value\":\"A\", \"label\": \"发现开发者\" }, { \"value\":\"B\", \"label\": \"发现开发者\" }, { \"value\":\"C\", \"label\": \"发现开发者\" }, { \"value\":\"D\", \"label\": \"发现开发者\" } ] }, { \"questionId\":\"2\", \"discraption\": \"你希望如何使用这款DAPP\", \"choice\": [ { \"value\":\"A\", \"label\": \"发现开发者\" }, { \"value\":\"B\", \"label\": \"发现开发者\" }, { \"value\":\"C\", \"label\": \"发现开发者\" }, { \"value\":\"D\", \"label\": \"发现开发者\" } ] } ]")
}

#[post("/user/postquestion")]
async fn post_question(user: web::Json<Questionmark>, pool: web::Data<AppState>) -> impl Responder {
    println!("接收到信息");

    let check_sql =
        " select count(id) as num  from sys_user_question where questionId = ? and  userId = ?  ";

    let res = sqlx::query_as::<MySql, QuestionNum>(check_sql)
        .bind(user.0.questionId)
        .bind(user.0.userId)
        .fetch_one(&pool.pool)
        .await;

    if res.is_ok() {
        if res.unwrap().num > 0 {
            //存在执行更新

            let update_sql =
            " update sys_user_question set choice = ? , updateTime = ? where questionId = ? and  userId = ? ";

            let insert_res = sqlx::query::<MySql>(update_sql)
                .bind(user.0.choice)
                .bind(user.0.updateTime)
                .bind(user.0.questionId)
                .bind(user.0.userId)
                .execute(&pool.pool)
                .await;
            match insert_res {
                Ok(result) => {
                    println!("{:?}", result);
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        } else {
            //不存在执行插入
            let sql = r#"insert into  sys_user_question(questionId,discraption,userId,updateTime,choice)
            VALUES (?,?,?,?,?)
            "#;
            let insert_res = sqlx::query::<MySql>(sql)
                .bind(user.0.questionId)
                .bind(user.0.discraption)
                .bind(user.0.userId)
                .bind(user.0.updateTime)
                .bind(user.0.choice)
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

        HttpResponse::Ok().body("回答成功")
    } else {
        HttpResponse::InternalServerError().body("更新错误")
    }

    // let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    // HttpResponse::Ok().body("body")
}

async fn execute_sql(
    res: Result<QuestionNum, Error>,
    user: web::Json<Questionmark>,
    pool: web::Data<AppState>,
) -> bool {
    match res {
        Ok(result) => {
            println!("{:?}", result.num);

            if result.num > 0 {
                //数据存在，执行更新

                true
            } else {
                //数据不存在，执行添加
                let sql = r#"insert into  sys_user_question(questionId,discraption,userId,updateTime,choice)
    VALUES (?,?,?,?,?)
    "#;
                let insert_res = sqlx::query::<MySql>(sql)
                    .bind(user.0.questionId)
                    .bind(user.0.discraption)
                    .bind(user.0.userId)
                    .bind(user.0.updateTime)
                    .bind(user.0.choice)
                    .execute(&pool.pool)
                    .await;

                false
            }
        }
        Err(err) => {
            println!("{:?}", err);
            false
        }
    }
}
