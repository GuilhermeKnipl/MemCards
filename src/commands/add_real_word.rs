use std::io::Write;

use colored::Colorize;

use crate::misc::trim_newline;
use crate::sql::query::{get_laststatus, insert_real_word, word_exist, wordlike_exist};
use crate::sql::connect::establish_connection;


pub fn run(word: &str){

    let conn = establish_connection();

    let exact_word = word_exist(&conn, word);
    let like_words = wordlike_exist(&conn, word);


    if like_words.unwrap_or(false){
        match exact_word{
            true => {

                let mut real_word = String::new();

                print!("\n{} {}: ","Define".bold(), &word.cyan().bold());

                std::io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut real_word)
                    .expect("Fail to read input");

                insert_real_word(&conn, word, trim_newline(&real_word));    


            }
            false => {
                get_laststatus(&conn, word, &false, false).unwrap();
                println!("\n{}","Word not found in Memory, Similar Words listed above".to_string().bold().bright_red());
            }
        }
    }
    else {
        print!("{:?},  {}", exact_word, &word);
        println!("Word not found in Memory");
    }


}
