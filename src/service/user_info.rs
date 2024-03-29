use actix_web::web::Json;
use actix_web::{error, HttpResponse, HttpResponseBuilder, HttpServer, App, web, Responder, get, post};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySql, Pool};
use crate::AppState;

#[derive(Deserialize, Debug,Clone,Serialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrU32 {
    String(String),
    U32(u32),
}

fn from_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match StrOrU32::deserialize(deserializer)? {
        StrOrU32::String(v) => v.parse().unwrap_or_default(),
        StrOrU32::U32(v) => v,
    })
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Wechatmark{
    #[serde(deserialize_with = "from_str")]
    id:  u32,   //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct ProfileMark{
    // #[serde(deserialize_with = "from_str")]
    id:  u32,   //要查询的微信号id
    nickname:Option<String>,
    identity:Option<String>,
    wechat:Option<String>,
    city:Option<String>,
    head_sculpture:Option<String>,
    tag1:Option<String>,
    tag2:Option<String>,
    wishtag:Option<Vec<String>>,
    introduce:Option<String>
}


#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct Visiblemark{
    id:u32 ,   //要查询的微信号id
    hide: u32 //1可见
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct WechatResponse{
    wechat:String    //要查询的微信号id
}

#[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
struct UserInfoSQL{
    email:Option<String>,
    profession:Option<String>,
    education:Option<String>,
    gender:Option<u32>,
    nickname: Option<String>, //要查询的微信号id
    identity:Option<String>,
    head_sculpture:Option<String>,
    id:u32,
    city:Option<String>,
    address:Option<String>,
    tag1:Option<String>,
    tag2:Option<String>,
    introduce:Option<String>,
    did:Option<String>,
}
#[derive(Debug,Clone,Serialize, Deserialize)]
struct UserInfoResponse{
    nickname:Option<String>, //要查询的微信号id
    identity:Option<String>,
    head_sculpture:Option<String>,
    id:u32,
    city:Option<String>,
    address:Option<String>,
    wishtag:Vec<String>,
    tag:Vec<String>,
    introduce:Option<String>,
    did:Option<String>,
}


#[derive(Debug,Clone,Serialize, Deserialize)]
struct UserProfileResponse{
    email:Option<String>,
    profession:Option<String>,
    education:Option<String>,
    gender:Option<u32>,
    nickname:Option<String>, //要查询的微信号id
    identity:Option<String>,
    head_sculpture:Option<String>,
    id:u32,
    city:Option<String>,
    address:Option<String>,
    wishtag:Vec<String>,
    tag:Vec<String>,
    introduce:Option<String>,
    did:Option<String>,
}

#[post("/user/info")]
async fn user_info( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    let id = user.0.id;
    println!("接收到信息{:?}",id);


    let sql = format!("select  email,profession,education,gender,nickname,identity,head_sculpture,id ,city,address,tag1,tag2,introduce,did from sys_user_info where id = {:?} ",id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,UserInfoSQL>(&sql).fetch_one(&pool.pool).await;

    match res {
        Ok(res) =>{

            let mut vec = Vec::new();
            let mut tag = Vec::new();
            vec.push("CEO".to_string());
            vec.push("Builder".to_string());
            let user_info = res;

            if user_info.tag1 == None{

            }else{
                tag.push(user_info.clone().tag1.unwrap());
                tag.push(user_info.clone().tag2.unwrap());
            }

        
            let user_res = UserInfoResponse{
                nickname:user_info.clone().nickname, //要查询的微信号id
                identity:user_info.clone().identity,
                head_sculpture:user_info.clone().head_sculpture,
                id:user_info.clone().id,
                city:user_info.clone().city,
                address:user_info.clone().address,
                wishtag:vec,
                tag:tag,
                introduce:user_info.clone().introduce,
                did:user_info.clone().did,
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


#[post("/user/profile")]
async fn user_profile( user: web::Json<Wechatmark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    let sql = format!("select email,profession,education,gender, nickname,identity,head_sculpture,id ,city,address,tag1,tag2,introduce,did from sys_user_info where id = {:?} ",user.0.id);
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,UserInfoResponse>(&sql).fetch_one(&pool.pool).await;
    let res = sqlx::query_as::< MySql,UserInfoSQL>(&sql).fetch_one(&pool.pool).await;

    match res {
        Ok(res) =>{

            let mut vec = Vec::new();
            let mut tag = Vec::new();
            vec.push("CEO".to_string());
            vec.push("Builder".to_string());
            let user_profile= res;

            if user_profile.tag1 == None{

            }else{
                tag.push(user_profile.clone().tag1.unwrap());
                tag.push(user_profile.clone().tag2.unwrap());
            }
        
            let user_res = UserProfileResponse{
                email:user_profile.clone().email,
                profession:user_profile.clone().profession,
                education:user_profile.clone().education,
                gender:user_profile.clone().gender,
                nickname:user_profile.clone().nickname, //要查询的微信号id
                identity:user_profile.clone().identity,
                head_sculpture:user_profile.clone().head_sculpture,
                id:user_profile.clone().id,
                city:user_profile.clone().city,
                address:user_profile.clone().address,
                wishtag:vec,
                tag:tag,
                introduce:user_profile.clone().introduce,
                did:user_profile.clone().did
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

#[post("/user/updateprofile")]
async fn update_profile( user: web::Json<ProfileMark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");
    let wtag = "";
    let sql = "update sys_user_info set nickname = ?, identity =?, tag1 = ?,tag2 =?, introduce = ? ,wechat = ?,city = ?  where id = ? ".to_string();

    let update_profile = sqlx::query::<MySql>(&sql)
    .bind(user.0.nickname)
    .bind(user.0.identity )
    .bind(user.0.tag1 )
    .bind(user.0.tag2 )
    .bind(user.0.introduce )
    .bind(user.0.wechat )
    .bind(user.0.city )
    .bind(user.0.id )
    .execute(&pool.pool)
    .await;
    println!("{:?}",update_profile);
   
  
    HttpResponse::Ok().body("success")
}



#[post("/user/changevisible")]
async fn change_visible( user: web::Json<Visiblemark>, pool: web::Data<AppState>) -> impl Responder {
    // format!("Hello {}!", name)
    println!("接收到信息");

    
    let sql = "update sys_user_info set hide = ?  where id = ?".to_string();
    println!("{:?}",sql.clone());
    // let res = sqlx::query_as::< _,WechatResponse>(&sql).fetch_one(&pool.pool).await;
    let _update_hide = sqlx::query::<MySql>(&sql)
            .bind(user.0.hide)
            .bind(user.0.id)
            .execute(&pool.pool)
            .await;
  
    HttpResponse::Ok().body("success")
}

