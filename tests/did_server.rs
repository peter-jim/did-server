use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use actix_files as fs;
use serde::{Serialize, Deserialize};
// use mysql::prelude::*;
// use mysql::*;
use sqlx::mysql::MySqlPoolOptions;
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

    //    match res {
    //         Ok(res) => {
    //             println!("正常 {:?}" ,res);
    //         }

    //         Err(res) => {
    //             println!("error")
    //         }
           
    //    }
    // //step 2 执行mysql
    // let url = "mysql://root:123456@114.55.67.80:3306/social";
    // let opts = Opts::from_url(url).unwrap(); // 类型转换将 url 转为opts
    //                                          //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    //                                          //创建连接池
    // let pool = Pool::new(opts).unwrap();
    // println!("start connect");
    // //连接数据库
    // let mut conn = pool.get_conn().unwrap();
    // println!("success connect");

    


    //取得 musql 结果，返回前10个



    format!("Hello {:?}!", res.unwrap())
}


#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    location:i32 ,   //实现下拉位置
    // age:Vec<i32>,   //年龄
    identity:i32,
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}


struct AppState{
    pool: Pool<MySql>
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











