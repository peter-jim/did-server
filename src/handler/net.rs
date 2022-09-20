'''
网络工具组件
'''

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

//路由指定路径（以 get 的方式）
#[get("/")]
async fn hello() -> impl Responder {
    //返回报文主体为 Hello,world 的 Http 响应
    HttpResponse::Ok().body("Hello,world!")
}

//将 async main 函数标记为 actix 系统的入口点。 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //创建 http 服务器
    HttpServer::new(|| {
        App::new()//新建一个应用
            .service(hello)//将 hello 函数作为服务
    })
    .bind("127.0.0.1:8080")?//绑定到指定的套接字地址
    .run()//开始监听
    .await
}