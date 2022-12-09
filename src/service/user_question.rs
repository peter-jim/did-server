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
    question_id: i32,
    discraption: String,
    choice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Questionmark {
    user_id: u32, //要查询的微信号id
    discraption: String,
    question_id: u32,
    update_time: String,
    choice: String,
}

//判断后台数据是否已存在
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct QuestionNum {
    num: i32,
}

#[get("/user/getquestion")]
async fn get_question(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select question_id,discraption,choice from sys_question ".to_string();
    let res = sqlx::query_as::<MySql, QuestionResponse>(&sql)
        .fetch_all(&pool.pool)
        .await;
    // let res:String = sqlx::query_scalar(r#"select aboutUs from sys_plate_config "#).fetch_one(&pool.pool).await;
    // println!("{:?}",res.unwrap());
    let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
}

#[post("/user/postquestion")]
async fn post_question(user: web::Json<Questionmark>, pool: web::Data<AppState>) -> impl Responder {
    println!("接收到信息");

    let check_sql =
        " select count(id) as num  from sys_user_question where question_id = ? and  user_id = ?  ";

    let res = sqlx::query_as::<MySql, QuestionNum>(check_sql)
        .bind(user.0.question_id)
        .bind(user.0.user_id)
        .fetch_one(&pool.pool)
        .await;

    if res.is_ok() {
        if res.unwrap().num > 0 {
            //存在执行更新

            let update_sql =
            " update sys_user_question set choice = ? , update_time = ? where question_id = ? and  user_id = ? ";

            let insert_res = sqlx::query::<MySql>(update_sql)
                .bind(user.0.choice)
                .bind(user.0.update_time)
                .bind(user.0.question_id)
                .bind(user.0.user_id)
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
            let sql = r#"insert into  sys_user_question(question_id,discraption,user_id,update_time,choice)
            VALUES (?,?,?,?,?)
            "#;
            let insert_res = sqlx::query::<MySql>(sql)
                .bind(user.0.question_id)
                .bind(user.0.discraption)
                .bind(user.0.user_id)
                .bind(user.0.update_time)
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
                let sql = r#"insert into  sys_user_question(question_id,discraption,user_id,update_time,choice)
    VALUES (?,?,?,?,?)
    "#;
                let insert_res = sqlx::query::<MySql>(sql)
                    .bind(user.0.question_id)
                    .bind(user.0.discraption)
                    .bind(user.0.user_id)
                    .bind(user.0.update_time)
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
