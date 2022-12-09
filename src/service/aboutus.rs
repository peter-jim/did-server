


use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Serialize, Deserialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct AboutusResponse{
     aboutus:String,
}


#[get("/user/aboutus")]
async fn about_us(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select aboutUs from sys_plate_config ".to_string();
    let res = sqlx::query_as::<MySql,AboutusResponse>(r#"select aboutus from sys_plate_config "#).fetch_one(&pool.pool).await;
    // let res:String = sqlx::query_scalar(r#"select aboutUs from sys_plate_config "#).fetch_one(&pool.pool).await;

    // println!("{:?}",res.unwrap());

    match res {
        Ok(res) =>{
            let body = serde_json::to_string(&res).unwrap();
   
            // return ;
            HttpResponse::Ok().body(body)
        }
        Err(res) =>{
            HttpResponse::InternalServerError().body("error")
        }
    }


    // let body = serde_json::to_string(&res.unwrap()).unwrap();
   
    // // return ;
    // HttpResponse::Ok().body(body)
    
}