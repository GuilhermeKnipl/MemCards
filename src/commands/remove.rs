use colored::Colorize;
use rusqlite::params;

use crate::sql::{connect::establish_connection, query::word_exist};



pub fn run(word:&str){
    let conn = establish_connection();

    if word_exist(&conn, word){
        let remove_query = "DELETE FROM last_status WHERE word = ?1";
        let remove_rec = "DELETE FROM records WHERE word = ?1";
        conn.execute(remove_query, params![word]).unwrap();
        conn.execute(remove_rec, params![word]).unwrap();
        println!("\n{}\n", format!("Sucessfully removed word: {}",word).green().bold());
    }
    else{
        println!("\n{}\n", format!("The word: {} are not found in Memory", word).red().bold());
    
    }

}
