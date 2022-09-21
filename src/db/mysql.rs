

#[derive(Debug,Clone)]
pub struct  UserInfo{
    pub id:i32,
    pub email:String,
    pub phone:String,
    pub login_id:i32,
    pub nickname:String,
    pub birthday:String,
    pub identity:String,
    pub  wechat:String,
    pub head_sculpture:String,
    pub  hide:String,
    pub gender:String,
    pub city:String,
    pub  profession:String,
    pub education:String,
    pub last_identity:String,
    pub create_time:String,
    pub update_time:String,
}




pub fn conn_mysql(){
     //设置连接字符串
     let url = "mysql://root:123456@114.55.67.80:3306/social";
     let opts = Opts::from_url(url).unwrap(); // 类型转换将 url 转为opts
                                              //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
                                              //创建连接池
     let pool = Pool::new(opts).unwrap();
     //连接数据库
     let mut conn = pool.get_conn().unwrap();
}




