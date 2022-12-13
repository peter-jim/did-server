use sqlx::{Pool, MySql};
use serde::{Deserialize, Serialize};
pub mod aboutus;
pub mod agreement;
pub mod didrecommand;
pub mod order_did;
pub mod privacy_policy;
pub mod user_info;
pub mod user_moment;
pub mod user_wechat;
pub mod recommend;
pub mod user_comment;
pub mod user_question;
pub mod user_frzs;
pub mod publish_check;
pub mod user_frzsnew;
pub mod user_tag;



pub struct AppState{
    pub pool: Pool<MySql>
}
