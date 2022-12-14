use actix_web::web::Json;
use actix_web::{
    error, get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

use crate::AppState;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Usermark {
    page: String,
    size: String,
    id: String,
    // location:String ,   //实现下拉位置
    // email:String,
    // age:(String,String),   //年龄
    // identity:String,   //身份，buider
    gender: Option<String>, // 0 代表女 ，1代表男，2代表全部
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct DIDsql {
    id: u32,
    city: Option<String>, //实现下拉位置
    // email:String,
    // age:Vec<i32>,   //年龄
    // tag:(String,String,String),
    identity: Option<String>,
    gender: Option<u32>, // 0 代表女 ，1代表男，2代表全部
    nickname: Option<String>,
    head_sculpture: Option<String>,
    update_time: Option<DateTime<Utc>>,
    address: Option<String>,
    publickey: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct DIDResponse {
    id: u32,
    city: Option<String>, //实现下拉位置
    // email:String,
    // age:Vec<i32>,   //年龄
    tag: (String, String, String),
    identity: Option<String>,
    gender: Option<u32>, // 0 代表女 ，1代表男，2代表全部
    nickname: Option<String>,
    headSculpture: Option<String>,
    updateTime: Option<DateTime<Utc>>,
    address: Option<String>,
    publickey: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct DIDPageElement {
    records: Vec<DIDResponse>,
    totalElements: u32,
    currentPage: u32,
    pageSize: u32,
    totalPages: u32,
    numberOfElements: u32,
}

#[post("/did")]
async fn didrecommand(name: web::Json<Usermark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    //step 0 . 判断参数是否为整数 “ ” 。合法性判断，避免sql注入攻击

    let mut sql:String = String::from(" select id,city,identity,gender,nickname,head_sculpture,update_time,address,publickey from sys_user_info") ;

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

    let page = name.0.page.parse::<u32>().unwrap();
    let size = name.0.size.parse::<u32>().unwrap();

    sql =  sql + &" limit ".to_string() + & (page*size).to_string()+ " , " + &((page*size) + size).to_string();

    println!("{:?}", &sql);

    let res = sqlx::query_as::<MySql, DIDsql>(&sql)
        .fetch_all(&pool.pool)
        .await;

    match res {
        Ok(res) => {
            let mut vec = Vec::new();
            vec.push("CEO".to_string());
            vec.push("Builder".to_string());
            vec.push("Builder".to_string());
            let user_info = res.clone();
            let mut user_vec: Vec<DIDResponse> = Vec::new();
            let mut total_element = 0u32;

            for i in 0..res.len() {
                let s = res[i].clone();

                let did = DIDResponse {
                    id: s.id,

                    city: s.city, //实现下拉位置
                    // email:String,
                    // age:Vec<i32>,   //年龄
                    tag: (
                        "Builder".to_string(),
                        "Builder".to_string(),
                        "Builder".to_string(),
                    ),
                    identity: s.identity,
                    gender: s.gender, // 0 代表女 ，1代表男，2代表全部
                    nickname: s.nickname,
                    headSculpture: s.head_sculpture,
                    updateTime: s.update_time,
                    address: s.address,
                    publickey: s.publickey,
                };
                user_vec.push(did.clone());

                total_element=total_element+1;
            }

            let did_res = DIDPageElement {
                records: user_vec,
                totalElements: total_element,
                currentPage: name.0.page.parse::<u32>().unwrap(),
                pageSize: name.0.size.parse::<u32>().unwrap(),
                totalPages: 0,
                numberOfElements: 0,
            };

            let body = serde_json::to_string(&did_res).unwrap();
            // return ;
            HttpResponse::Ok().content_type("application/json").body(body)
        }
        Err(res) => HttpResponse::InternalServerError().body("error"),
    }
}

// impl DIDsql {
//     pub fn to_response(&self) -> DIDResponse {
//         DIDResponse {
//             id: self.id,
//             city: self.city,
//             tag: ("hello".to_string(),"hello".to_string(),"hello".to_string()),
//             identity:self.identity,
//             gender: self.gender,
//             nickname: self.nickname,
//             head_sculpture: self.head_sculpture,
//             update_time: self.update_time,
//             address: self.address,
//             publickey: self.publickey,
//         }
//     }
// }
