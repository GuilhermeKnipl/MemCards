
use chrono::Local;

pub fn now_datetime() -> String{
    let now_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
    return now_dt.to_string()
}
