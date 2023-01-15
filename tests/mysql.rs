// use mysql::{Opts, Pool};
// use mysql::prelude::*;
// use mysql::*;

use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, FromRow};
use tokio;

#[test]
fn conn_mysql() {
    //设置连接字符串
    //     let url = "mysql://root:123456@114.55.67.80:3306/social";
    //     let opts = Opts::from_url(url).unwrap(); // 类型转换将 url 转为opts
    //                                              //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    //                                              //创建连接池
    //     let pool = Pool::new(opts).unwrap();
    //     println!("start connect");
    //     //连接数据库
    //     let mut conn = pool.get_conn().unwrap();
    //     println!("success connect");

    //     //方式2：将数据集取出存储在Vec中
    //     let res:Vec<(i32,String)>=conn.query("Select id,email from sys_user_info").unwrap();
    //     for r in res{
    //         println!("id={},name={}",r.0,r.1);
    //     }

    //     //方式3：将数据转换成Struct
    //     struct User{
    //       id:i32,
    //       email:String,
    //   }

    //   let res=conn.query_map("Select id,email from sys_user_info user",
    //           |(id,email)|User{
    //               id,
    //               email
    //               }
    //           ).expect("QUERY FAILED");
    //   for user in res{
    //       println!("id={},name={},",user.id,user.email);
    //   }
}

#[tokio::test] // or #[tokio::main]
async fn test_get_count() {

    #[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
    struct QuestionNum{
        num:i32
    }

     
    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .connect("xxx")
        .await
        .unwrap_or_else(|_| std::process::exit(0x0100));

    let check_sql =
        " select count(id) as num  from sys_user_question where question_id = ? and  user_id = ?  ";

    let res = sqlx::query_as::<MySql,QuestionNum>(check_sql).bind(1).bind(1).fetch_one(&pool).await;

    match res {
        Ok(result) => {
            println!("{:?}", result.num);
            
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}



#[tokio::test] // or #[tokio::main]
async fn test_update_question() {

    #[derive(Debug,Clone,Serialize, Deserialize,FromRow)]
    struct QuestionNum{
        num:i32
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .connect("mysql://test:123456@114.55.67.80:3306/social")
        .await
        .unwrap_or_else(|_| std::process::exit(0x0100));

    let update_sql =
        " update sys_user_question set choice = ? , update_time = ? where question_id = ? and  user_id = ? ";

    let res = sqlx::query(update_sql)
    .bind("测试回答")
    .bind("2022-11-16 11:20:43")
    .bind(1)
    .bind(1)
    .execute(&pool).await;

    match res {
        Ok(result) => {
            println!("{:?}", result);
            
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}



#[test]


fn type_time(){



  

    


}
