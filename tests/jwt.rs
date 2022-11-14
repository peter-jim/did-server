
use jsonwebtoken::errors::ErrorKind;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
 
// /// 我们的声言结构型, 需要由`Serialize` 或 `Deserialize`派生
// #[derive(Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct Claims {
//     pub sub: String,
//     pub exp: u64,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "rocket::serde")]
// pub struct UserAuth {
//     pub id: i32,
//     pub key: String,
// }

// const PASSWORD: &str="secret";
pub const TOKEN_SECRET:&str = "0fd2af6f";


#[derive(Debug,Serialize,Deserialize)]
struct Login{
    password:String,
}


#[get("/secret")]
async fn authed() -> impl Responder{
    // login(actix_web::web::Json(Login{password:b"secret"}));
    format!("you are authenticated")
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    email: String,
    exp: usize,
}
#[get("/login")]
async fn login(login: web::Json<Login>)-> impl Responder{
    let mut  PASSWORD: &str="!DAR$*w3social*/!nu4r@tte@";
    if &login.password == PASSWORD{
        let claims = Claims{
            user_id: "1".to_string(),
            email: "lancedeng".to_string(),
            exp: 10000000000,
        };

        let token  = encode(&Header::default(), &claims, &EncodingKey::from_secret(PASSWORD.as_bytes())).unwrap();
        println!("{:?}",token );


        let mut validation = Validation::new(Algorithm::HS256);
        let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(PASSWORD.as_bytes()), &validation) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
        };
       
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);

        format!("token {:?}",token)
        // println!("token is {:?}",token);
    }else {
        format!("you are unauthenticated")   
    }
}

#[actix_web::test] // or #[tokio::main]
async fn main() -> std::io::Result<()> {


    print!("服务启动");

    HttpServer::new(move|| {
        App::new()   //公共数据传入一个结构体，方便操作
        .service(authed)
        .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




