use std::io::{self, Write};
use colored::*;



pub fn run(){
    let fail_inpt: &str = "Fail to read your input";    

    print!("\n{}: ", "Add a new word".trim().yellow().bold());

    io::stdout().flush().unwrap();

    let mut new_word: String = String::new();
    io::stdin()
        .read_line(&mut new_word)
        .expect(fail_inpt);

    print!("\n{} ", format!("Add a meaning to the word:").trim().yellow().bold());

    io::stdout().flush().unwrap();

    let mut pred_word : String = String::new();
    
    io::stdin()
        .read_line(&mut pred_word)
        .expect(fail_inpt);

    println!("{}", format!("{} = {}"
        , new_word.trim().bright_blue().bold()
        , pred_word.trim().green().bold()).bold());  
    

    //println!("{}", format!("Input is {}", input).bold().blue())

    // real_word
}

