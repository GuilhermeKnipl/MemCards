
use chrono::Local;

pub fn now_datetime() -> String{
    let now_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
    return now_dt.to_string()
}

pub fn now_date_file_fmt() -> String {
    let now_fmt = Local::now().format("%Y_%m_%d");
    return now_fmt.to_string();
}

pub fn trim_newline(word: &str) -> &str {
    word.trim_matches('\n')
}
