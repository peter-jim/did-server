// use mysql::{Opts, Pool};
// use mysql::prelude::*;
// use mysql::*;

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

#[test]
fn get_userinfo() {
    

  
  
}



