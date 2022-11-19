

use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis::aio::Connection;
use mobc_redis::{redis};



#[tokio::test] // or #[tokio::main]
async fn test_redis_con() {

    //redis://:password@ip:port/
    let client = redis::Client::open("redis://:S6E481r623tn13PF@114.55.67.80:6379/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(20).build(manager);


    let mut conn = pool.get().await.unwrap();

    let s:String = redis::cmd("PING") .query_async(&mut conn as &mut Connection).await.unwrap();

    println!("{:?}", s); // "PONG"


}
