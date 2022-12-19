use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Serialize, Deserialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Wechatmark{
    id:String ,   //要查询的微信号id
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct WechatResponse{
    wechat:String    //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserInfoSQL{
    nickname: Option<String>, //要查询的微信号id
    identity:Option<String>,
    head_sculpture:Option<String>,
    id:u32,
    city:Option<String>,
    address:Option<String>,
    tag1:String,
    tag2:String,
    introduce:String
}
#[derive(Debug,Clone,Serialize, Deserialize)]
struct UserInfoResponse{
    nickname:String, //要查询的微信号id
    identity:String,
    head_sculpture:String,
    id:u32,
    city:String,
    address:String,
    wishtag:Vec<String>,
    tag:Vec<String>,
    introduce:String
}


#[post("/user/info")]
async fn user_info( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select nickname,identity,head_sculpture,id ,city,address,tag1,tag2,introduce from sys_user_info where id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,UserInfoSQL>(&sql).fetch_one(&pool.pool).await;
    
    // res[]
    // for i in res{
    //     println!("{:?}",i."row");
    // }
    // println!("{:?}",res);
    // format!("{:?}", serde_json::to_value(&res.unwrap())  )
   

    match res {
        Ok(res) =>{

            let mut vec = Vec::new();
            let mut tag = Vec::new();
            vec.push("CEO".to_string());
            vec.push("Builder".to_string());
            let user_info = res;

            tag.push(user_info.clone().tag1);
            tag.push(user_info.clone().tag2);
        
            let user_res = UserInfoResponse{
                nickname:user_info.clone().nickname.unwrap(), //要查询的微信号id
                identity:user_info.clone().identity.unwrap(),
                head_sculpture:user_info.clone().head_sculpture.unwrap(),
                id:user_info.clone().id,
                city:user_info.clone().city.unwrap(),
                address:user_info.clone().address.unwrap(),
                wishtag:vec,
                tag:tag,
                introduce:user_info.clone().introduce
            };
            let body = serde_json::to_string(&user_res).unwrap();
            // return ;
            HttpResponse::Ok().content_type("application/json").body(body)
        }
        Err(res) =>{
            HttpResponse::InternalServerError().body("error")
        }
    }
    
}
