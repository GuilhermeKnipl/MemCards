use rusqlite::{Connection,Result};
use directories::ProjectDirs;
use std::{fs::{self, File},path::PathBuf, process::exit};

pub fn create_tables(path: &PathBuf) -> Result<()> {
    let conn = fast_connection(path);

    // `last_status` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS last_status (
            creation_date DATETIME,
            word TEXT PRIMARY KEY,
            pred_word TEXT,
            real_word TEXT,
            last_search DATETIME,
            context TEXT
        )",
        [],
    )?;

    //`review` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS review (
            creation_date DATETIME,
            review_date DATETIME,
            word TEXT,
            score INTEGER,
            notes TEXT
        )",
        [],
    )?;

    // `records` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS records (
            search_date DATETIME,
            word TEXT,
            pred_word TEXT
        )",
        [],
    )?;
    Ok(())
}


pub fn get_path() -> PathBuf {
    let proj_dir = ProjectDirs::from("com", "lynx" , "MemCards")
        .expect("Failed to find config dir");

    let dir_data = proj_dir.data_dir();
    fs::create_dir_all(dir_data).unwrap();
    let db_path = dir_data.join("memcards.db");
    if db_path.exists(){

        return db_path;
    }
    else{
        match File::create(&db_path){
            Ok(_file)=>{}
            Err(e) => {
                println!("{e}");
                exit(1);
            }
        }

        create_tables(&db_path).unwrap();

        return db_path;
    }

}

fn fast_connection(path: &PathBuf) -> Connection{
    let conn = Connection::open(path).unwrap();
    return conn
}


pub fn establish_connection() -> Connection{
    let db_path = get_path();
    let conn = Connection::open(db_path).unwrap(); 
    
    return conn
}
