use colored::Colorize;

use crate::sql::{connect::establish_connection, query::{get_laststatus, wordlike_exist}};






pub fn run(word: &str , sort_asc:&bool, _limit: &u8, _records: &bool, detail: &bool){
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
