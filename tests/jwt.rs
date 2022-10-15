
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



