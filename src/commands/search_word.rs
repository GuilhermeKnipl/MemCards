use colored::Colorize;

use crate::sql::{mConnect::establish_connection, mQuery::{get_laststatus, word_exist, wordlike_exist}};






pub fn run(word: &str , sort_asc:&bool, limit: &u8, records: &bool, detail: &bool){
    let conn = establish_connection();


    match  wordlike_exist(&conn, word).unwrap_or(false){
        true => {
            get_laststatus(&conn, word, sort_asc, detail.to_owned()).unwrap();
        }
    
    
        false => {
            println!("{}", "Doesn't Exist".red().bold());
        }    
}    
 

/*
    if records{
        println!("records true");
    }
    else {
       println!("records false"); 
    }
*/
}
