use actix_web::{get, web, App, HttpServer, Responder, post};
use serde::{Serialize, Deserialize};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("接收到信息");
    format!("Hello {}!", name)
}


#[post("/did")]
async fn didrecommand(name: web::Json<Usermark>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    
    let mut sql:String = String::from("xxx") ;

    sql = sql + &"123" ;

    //处理sql语句
    println!("{:?}",name.age[0]);
    println!("{:?}",name.age[1]);

    
    match name.gender {

        0i32 =>{
            println!("xxx")
        }

        1i32 =>{
            println!("xxx")
        }

        2i32 =>{
            println!("xxx")
        }

        _ =>{
            todo!()
        }
    }

     for i in &name.identity{    
        match i {
            
            0i32 =>{
                println!("xxx")
            }

            1i32 =>{
                println!("xxx")
            }

            2i32 =>{
                println!("xxx")
            }

            4i32 =>{
                println!("xxx")
            }
            5i32 =>{
                println!("xxx")
            }

            _ =>{
                todo!()
            }
        }
     }   


    let sql = "select id,email from sys_user_info where age >=  {} and age <= {} and gender = {} and identity = {} ";

    //step 2 执行mysql

    //取得 musql 结果，返回前10个



    format!("Hello {:?}!", name)
}


#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    location:i32 ,   //实现下拉位置
    age:Vec<i32>,
    identity:Vec<i32>,
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}


#[actix_web::test] // or #[tokio::main]
async fn test() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
        .service(didrecommand)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}