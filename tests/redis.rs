

use std::collections::HashMap;

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

    let s:String = redis::cmd("PING").query_async(&mut conn as &mut Connection).await.unwrap();
    println!("{:?}", s);// "PONG


}


#[tokio::test] // or #[tokio::main]
async fn get_redis() {

    //redis://:password@ip:port/
    let client = redis::Client::open("redis://:S6E481r623tn13PF@114.55.67.80:6379/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(20).build(manager);

    let mut conn = pool.get().await.unwrap();

    let s:String = redis::cmd("GET").arg("test_key") .query_async(&mut conn as &mut Connection).await.unwrap();
    println!("{:?}", s);// "PONG
}


#[tokio::test] // or #[tokio::main]

async fn set_redis() {

    //redis://:password@ip:port/
    let client = redis::Client::open("redis://:S6E481r623tn13PF@114.55.67.80:6379/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(20).build(manager);

    let mut conn = pool.get().await.unwrap();
   
    let s:String = redis::cmd("SET").arg("test_key").arg(42) .query_async(&mut conn as &mut Connection).await.unwrap();
    println!("{:?}", s);// "PONG

}


#[tokio::test] // or #[tokio::main]
async fn hset_redis() {

    //redis://:password@ip:port/
    let client = redis::Client::open("redis://:S6E481r623tn13PF@114.55.67.80:6379/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(20).build(manager);

    let mut conn = pool.get().await.unwrap();
   
    let s:HashMap<String, String> = redis::cmd("HSET").arg("moment").arg("rust").arg(42) .query_async(&mut conn as &mut Connection).await.unwrap();
    println!("{:?}", s);// "PONG

}


#[tokio::test] // or #[tokio::main]
async fn hget_redis() {

    //redis://:password@ip:port/
    let client = redis::Client::open("redis://:S6E481r623tn13PF@114.55.67.80:6379/").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(20).build(manager);

    let mut conn = pool.get().await.unwrap();
   
    //如果查询结果不在，会报错。
    let s:Result<String, redis::RedisError>= redis::cmd("HGET").arg("moment").arg("field2") .query_async(&mut conn as &mut Connection).await;
    println!("{:?}", s);// "PONG



}










