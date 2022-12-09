use chrono::{self, Local};







#[tokio::test] // or #[tokio::main]
async fn get_local_time() {

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
}