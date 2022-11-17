use std::fmt::Error;

use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use actix_files as fs;
use serde::{Serialize, Deserialize};
// use mysql::prelude::*;
// use mysql::*;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("接收到信息");
    format!("Hello {}!", name)
}


#[post("/did")]
async fn didrecommand(name: web::Json<Usermark>,pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    //step 0 . 判断参数是否为整数 “ ” 。合法性判断，避免sql注入攻击 


    let mut sql:String = String::from("select id,email from sys_user_info where ") ;
    
    match name.gender {

        0i32 =>{
            sql =sql +&"gender = 0".to_string()
            // println!("gender")
        }

        1i32 =>{
            sql = sql +&"gender = 1".to_string()
        }

        2i32 =>{
             println!("{:?}",sql);
        }

        _ =>{
            todo!()
        }
    }

    
        match name.identity {
            
            0i32 =>{
                sql = sql +  &" and identity = 0".to_string()
            }

            1i32 =>{
                sql = sql + &" and identity = 2".to_string()
            }

            2i32 =>{
                sql = sql +  &" and identity = 3".to_string()
            }

            4i32 =>{
                sql = sql +  &" and identity = 4".to_string()
            }
            999i32 =>{
                // sql = sql +  &" and identity = 0".to_string()
                println!("{:?}",sql);
            }

            _ =>{
                todo!()
            }
        }
        
        sql =  sql + &" limit ".to_string() + &name.location.to_string()+ " , " + &(name.location + 10).to_string();

        
        println!("{:?}",&sql);
        

       let res = sqlx::query::<MySql>("select id,email from sys_user_info").fetch_all(&pool.pool).await;

   



    format!("Hello {:?}!", res.unwrap())
}


#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    location:i32 ,   //实现下拉位置
    // age:Vec<i32>,   //年龄
    identity:i32,
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Wechatmark{
    id:u32 ,   //要查询的微信号id
}


struct AppState{
    pool: Pool<MySql>
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct WechatResponse{
    wechat:String    //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserInfoResponse{
    nickname:String, //要查询的微信号id
    identity:String,
    head_sculpture:String,
    id:u32,
    city:String
    // wishtag:String
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserMomentResponse{
    user_id:i32,
    content:String,
    likes:i32,
    img:String,
    // tah:Json<>,
    page_view:i32
    // create_time:
}



#[actix_web::test] // or #[tokio::main]
async fn test() -> std::io::Result<()> {

    let pool = MySqlPoolOptions::new()
    .max_connections(50)

    .connect("mysql://test:123456@114.55.67.80:3306/social")
    .await.unwrap_or_else(|_| { std::process::exit(0x0100) });

    

    HttpServer::new(move|| {
        App::new().app_data( web::Data::new( AppState{pool:pool.clone() } ))   //公共数据传入一个结构体，方便操作
        .service(greet)
        .service(didrecommand)
        .service(iamge)
        .service(agreement)
        .service(privacy_policy)
        .service(aboutus)
        .service(order_did)
        .service(user_wechat)
        .service(user_info)
        .service(user_moment)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


//测试返回图片
#[get("/image")]
async fn iamge() -> Result<fs::NamedFile, std::io::Error> {
    println!("接收到信息");
    // format!("Hello {}!")

    fs::NamedFile::open("tests/crypto.jpeg")

}


#[get("/user/agreement")]
async fn agreement(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select userAgreement from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

#[get("/user/privacy")]
async fn privacy_policy(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select privacyPolicy from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

#[get("/user/aboutus")]
async fn aboutus(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select aboutUs from sys_plate_config ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

//获取正在发行的did
#[get("/order/did")]
async fn order_did(pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let sql = "select circulation,price,title,discraption from sys_did where status = 1 ".to_string();
    let res = sqlx::query::<MySql>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res.unwrap())
    
}

#[get("/user/wechat")]
async fn user_wechat( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select wechat from sys_user_info where id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    let res = sqlx::query_as::< _,WechatResponse>(&sql)
    
    .fetch_one(&pool.pool).await.unwrap();
    
    // res[]

    // for i in res{
    //     println!("{:?}",i."row");
    // }
    println!("{:?}",res.wechat);
    format!("{:?}!", 1)
    
}

#[get("/user/info")]
async fn user_info( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select nickname,identity,head_sculpture,id ,city from sys_user_info where id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    
    // res[]

    // for i in res{
    //     println!("{:?}",i."row");
    // }
    // println!("{:?}",res);
    format!("{:?}!", res)
    
}


#[get("/user/moment")]
async fn user_moment( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select id,user_id, content,likes,img,page_view from sys_moment where id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,UserMomentResponse>(&sql).fetch_all(&pool.pool).await;
    format!("{:?}!", res)


   
    
}








