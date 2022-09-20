'''

mysql 工具组件

'''
pub fn init_mysql() -> PooledConn {
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let url = v["mysql"].as_str().unwrap();
    // println!("初始化muysql");
    //设置连接字符串
    // let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap(); // 类型转换将 url 转为opts
                                             //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}



