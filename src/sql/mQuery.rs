use colored::Colorize;
use rusqlite::{params, Connection, Result};
use std::error::Error;

use crate::misc::now_datetime;


#[derive(Debug)]
pub struct LastStatus{
    creation_date : String,
    word : String,
    pred_word: String,
    real_word: String,
    last_search: String, 
    context: String
}
/*
pub struct LastStatusSearch{
    creation_date : String,
    word : String,
    pred_word: String,
    real_word: String,
    last_search: String, 
    context: String
}
*/

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

    let mut query = conn.prepare("SELECT word FROM last_status WHERE LOWER(word) like LOWER(?) LIMIT 1").unwrap();
    let exist = query.exists(params![format!("{}", word)]).unwrap_or(false);
    return exist
      
}

pub fn wordlike_exist(conn: &Connection, word: &str) -> Result<bool>{

     let mut query = conn.prepare("SELECT word FROM last_status WHERE LOWER(word) like LOWER(?) LIMIT 1")?;
     let exist = query.exists(params![format!("%{}%", word)])?;

     Ok(exist)
}


pub fn add_word(conn: &Connection, word: &str, pred_word: &str, context: &str){
    let now_dt = now_datetime();
    let last_search = now_dt.clone();

    if word_exist(&conn, word){
        let ls_update_query = "UPDATE last_status SET pred_word = ?1, last_search= ?2, context = ?3
        WHERE word = ?4";
        conn.execute(ls_update_query, params![pred_word, last_search, context, word]).unwrap();
    }

    else {
        let last_status_query = "INSERT INTO last_status(creation_date, word, pred_word, last_search, context) 
        VALUES(?1, ?2, ?3, ?4, ?5)";
        conn.execute(last_status_query, params![now_dt, word, pred_word, last_search, context]).unwrap();

    }

    let records_query = "INSERT INTO records(search_date, word, pred_word) VALUES(?1, ?2, ?3)";



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


pub fn get_records(conn: &Connection, search_word: &str , sort_asc: bool) -> Result<Vec<Records>, Box<dyn Error>>{    
   
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
pub fn get_laststatus(conn: &Connection, search_word: &str , sort_asc: &bool, detail: bool) -> Result<Vec<LastStatus>, Box<dyn Error>>{    
   
    let sort;

    if sort_asc.to_owned() {
        sort = "ASC";
    } else {
        sort = "DESC";
    }

    let query = format!("SELECT word, pred_word, real_word ,strftime('%Y-%m-%d',creation_date), 
        last_search, context FROM last_status 
        WHERE LOWER(word) like LOWER(?1) ORDER BY creation_date {}", sort);

    let mut select = conn.prepare(&query).unwrap();

    let laststatus = select.query_map(params![format!("%{}%", search_word)], |row|{
        Ok( LastStatus{
            word: row.get(0)?,
            pred_word: row.get(1)?,
            real_word: row.get(2).unwrap_or("".to_string()),
            creation_date: row.get(3)?,
            last_search: row.get(4)?,
            context: row.get(5)?
        })
        
    })?
    .collect::<Result<Vec<_>, _>>()?;

    let word_column_width = 15;
    let date_column_width = 7;
    
    for row in &laststatus{
        print!("-> {:width$} ", row.word.trim().white().bold(), width = word_column_width);
        println!("{:<width_date$}", row.creation_date.trim().bright_white(), width_date = date_column_width);
        if detail{
            println!(" - Predicted: {}", format!("{}", row.pred_word.trim().cyan()));
            println!(" - Real Meaning: {}", format!("{}", row.real_word.trim().cyan()));
            println!(" - Context: {}", format!("{}", row.context.trim().cyan()));
        }
    }


    Ok(laststatus)
}




pub fn insert_in_last_status(conn: Connection,values: LastStatus){
    conn.execute(
    "INSERT INTO last_status(creation_date, word, pred_word, last_search, context) 
        VALUE (?1, ?2, ?3, ?4, ?5)"
        ,params![values.creation_date,
            values.word,values.pred_word,
            values.last_search,values.context]).unwrap();
}




