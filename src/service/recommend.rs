
use actix_web::{get, web, App, HttpServer, Responder, post};
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize, Deserialize)]
struct Usermark{
    age:Vec<i32>,
    identity:Vec<i32>,
    gender:i32,   // 0 代表女 ，1代表男，2代表全部
}

#[post("/did")]
async fn didrecommand(user: web::Json<Usermark>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    // format!("Hello {:?}!", name.)


    //将请求处理成sql语句
    let min_age = user.age[0];
    let max_age = user.age[1];

    






}



// fn Recommandfilter(user) -> Vec<> {

// }