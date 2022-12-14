use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Serialize, Deserialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct TagResponse{
    id:i32,
    tag:String,
}

// #[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
// struct DataTagResponse{
//     data:Vec<TagResponse> 
// }




#[get("/user/tag")]
async fn user_tag(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select id,tag from sys_tag ".to_string();
    let res = sqlx::query_as::< MySql,TagResponse>(&sql).fetch_all(&pool.pool).await;
    
    match res {
        Ok(res) =>{
            // let tag = DataTagResponse{
            //     data:res
            // };
            let body = serde_json::to_string(&res).unwrap();
            HttpResponse::Ok().content_type("application/json") .body(body)
        }
        Err(res) =>{
            HttpResponse::InternalServerError().body("error")
        }
    }
    
}
