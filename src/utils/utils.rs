

use chrono::Local;

fn get_local_time() -> String  {

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    return now
}



