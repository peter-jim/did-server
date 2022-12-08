use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;
#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    id:i32,
    location:i32 ,   //实现下拉位置
    email:String,
    // age:Vec<i32>,   //年龄
    identity:i32,   //身份，buider
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct DIDReponse{
    id:u32,
    city:String ,   //实现下拉位置
    // email:String,
    // age:Vec<i32>,   //年龄
    identity:String,
    gender:u32,   // 0 代表女 ，1代表男，2代表全部
    nickname:String,
    head_sculpture:String,
    update_time:DateTime<Utc>,
    address:String,
    publickey:String
}



#[post("/did")]
async fn didrecommand(name: web::Json<Usermark>,pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    //step 0 . 判断参数是否为整数 “ ” 。合法性判断，避免sql注入攻击 


    let mut sql:String = String::from("select id,city,identity,gender,nickname,head_sculpture,update_time,address,publickey from sys_user_info  ") ;
    
    // match name.gender {

    //     0i32 =>{
    //         sql =sql +&"gender = 0".to_string()
            
    //         // println!("gender")
    //     }

    //     1i32 =>{
    //         sql = sql +&"gender = 1".to_string()
    //     }

    //     2i32 =>{
    //          println!("{:?}",sql);
    //     }

    //     _ =>{
    //         todo!()
    //     }
    // };

    
        // match name.identity {
            
        //     0i32 =>{
        //         sql = sql +  &" and identity = 0".to_string()
        //     }

        //     1i32 =>{
        //         sql = sql + &" and identity = 2".to_string()
        //     }

        //     2i32 =>{
        //         sql = sql +  &" and identity = 3".to_string()
        //     }

        //     4i32 =>{
        //         sql = sql +  &" and identity = 4".to_string()
        //     }
        //     999i32 =>{
        //         // sql = sql +  &" and identity = 0".to_string()
        //         println!("{:?}",sql);
        //     }

        //     _ =>{
        //         todo!()
        //     }
        // }
        
        // sql =  sql + &" limit ".to_string() + &name.location.to_string()+ " , " + &(name.location + 10).to_string();

        
        println!("{:?}",&sql);
        

    let res = sqlx::query_as::< MySql,DIDReponse>(&sql).fetch_all(&pool.pool).await;

    let body = serde_json::to_string(&res.unwrap()).unwrap();
    // return ;
    HttpResponse::Ok().body(body)
}
