use clap::Parser;
//use colored::Colorize;
use mem_cards::client::Cli;
//use mem_cards::misc::now_datetime;
use mem_cards::sql::{mQuery,mConnect};

fn main() {

    let conn = mConnect::establish_connection();
    
    //mQuery::add_word(&conn, "test2", "thanksa","adas");
    println!("{:?}",mQuery::word_exist(&conn, "test2"));
    println!("{:?}" ,mQuery::word_exist(&conn, "test32"));
    //mQuery::get_records(conn, "test2",false).unwrap();
    //println!("{}", creation_date.bold());
    //conn    

    let cli: Cli= Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error {}", e);
        std::process::exit(1);

    }
}

