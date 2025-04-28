use std::io::{self, Write};
use colored::*;
use crate::sql::connect::establish_connection;
use crate::sql::query::{word_exist, add_word};
//use crate::misc::now_datetime;




pub fn run(){

    let fail_inpt: &str = "Fail to read your input";    

    let conn = establish_connection();

    let mut new_word: String = String::new();
    let mut pred_word : String = String::new();
    let mut context : String = String::new();


    print!("\n{}: ", "# Add a new word".trim().white().bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut new_word)
        .expect(fail_inpt);



    print!("{} ", format!("## Add a meaning to the word:").trim().white().bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut pred_word)
        .expect(fail_inpt);


    print!("{} ", format!("### Add a context to the word(OPTIONAL):").trim().white().bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut context)
        .expect(fail_inpt);




    if word_exist(&conn, &new_word){
        println!("\n Old Meaning: \n New Meaning");

    } else {
        println!("{}",format!("New word {} Added To Memory", &new_word.trim()).green().bold());
    }

    add_word(&conn, &new_word, &pred_word, &context);

    //get_records(&conn, &new_word, true).unwrap();
    
    conn.close().unwrap();

}

