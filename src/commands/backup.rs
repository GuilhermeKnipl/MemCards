use std::{fs, path::PathBuf};

use crate::{misc::now_date_file_fmt, sql::connect::get_path};

pub fn run(end_path:&str){
    let db_path = get_path();
    let mut end_path = PathBuf::from(end_path);

    if end_path.is_dir(){
        let backup_fname = format!("backup_{}.db", now_date_file_fmt());
        end_path.push(backup_fname);
    }

    match fs::copy(db_path, end_path) {
        Ok(_) => {
            println!("{}", format!("Backup of day {}", now_date_file_fmt()));
        }
        Err(e) => {
            eprintln!("Fail to Backup \n{}", e);
        }
    }
}
