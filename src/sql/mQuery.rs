use colored::Colorize;
use rusqlite::{params, Connection, Result};
use std::error::Error;

use crate::misc::now_datetime;


#[derive(Debug)]
pub struct LastStatus{
    creation_date : String,
    word : String,
    pred_word: String,
    _real_word: String,
    last_search: String, 
    context: String
}

#[derive(Debug)]
pub struct Records{
    search_date: String,
    word: String,
    pred_word: String,

}

//pub struct NewWord{}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Review {
    review_date: String,
    word: String,
    score: i32,
    notes: String,
}


pub fn word_exist(conn: &Connection, word: &str)-> bool{
    
    match get_creation_date(conn, word){
       Ok(_) => true,
       Err(_) => false
    }

}


pub fn add_word(conn: &Connection, word: &str, pred_word: &str, context: &str){
    let now_dt = now_datetime();
    let creation_date: String;    
    let last_search = now_dt.clone();

    if word_exist(&conn, word){
        creation_date = get_creation_date(conn, word).unwrap();
    }
    else {
        creation_date  = now_dt.clone();
    }

    let records_query = "INSERT INTO records(search_date, word, pred_word) VALUES(?1, ?2, ?3)";
    let last_status_query = "INSERT INTO last_status(creation_date, word, pred_word, last_search, context) 
        VALUES(?1, ?2, ?3, ?4, ?5)";

    conn.execute(last_status_query, params![creation_date, word, pred_word, last_search, context]).unwrap();
    conn.execute(records_query, params![last_search, word, pred_word]).unwrap();
    


    //word
    //pred_word
    //context
    //last_search
}


pub fn get_creation_date(conn: &Connection, word: &str) -> Result<String,Box<dyn std::error::Error>>{

    let query = "SELECT search_date, word, pred_word FROM records 
        WHERE LOWER(word) == ?1 ORDER BY search_date ASC";

    let mut query_prep = conn.prepare(query)?;

    let search_date= query_prep.query_row(params![word], |row| row.get(0))?;
        
    Ok(search_date)
}


pub fn get_records(conn: Connection, search_word: &str , sort_asc: bool) -> Result<Vec<Records>, Box<dyn Error>>{    
   
    let sort;

    if sort_asc {
        sort = "ASC";
    } else {
        sort = "DESC";
    }

    let query = format!("SELECT search_date, word, pred_word FROM records 
        WHERE LOWER(word) == ?1 ORDER BY search_date {}", sort);

    let mut select = conn.prepare(&query).unwrap();

    let records = select.query_map(params![search_word], |row|{
        Ok( Records{
            search_date: row.get(0)?,
            word: row.get(1)?,
            pred_word: row.get(2)?
        })
        
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    for row in &records{
        print!("{} ", format!("{}", row.search_date.bright_cyan().bold()));
        print!("{} ", format!("{}", row.word.bright_cyan().bold()));
        print!("{} \n", format!("{}", row.pred_word.bright_cyan().bold()));
    }


    Ok(records)
}

pub fn insert_in_last_status(conn: Connection,values: LastStatus){
    conn.execute(
    "INSERT INTO last_status(creation_date, word, pred_word, last_search, context) 
        VALUE (?1, ?2, ?3, ?4, ?5)"
        ,params![values.creation_date,
            values.word,values.pred_word,
            values.last_search,values.context]).unwrap();
}




