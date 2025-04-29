use colored::Colorize;

use crate::sql::{connect::establish_connection, query::{get_last_status_total, get_records_count}};

pub fn run(word: &str, count: u8) {
    let conn = establish_connection();
    let r_count = get_records_count(&conn, word);
    let total_count = get_last_status_total(&conn);
    println!("-| {}", format!("{} records of the word: {word}",r_count).bold().blue());

    println!("--| {}", format!("Total unique words in Memory:{}",total_count).bold().blue())

}
