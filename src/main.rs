use clap::Parser;
//use colored::Colorize;
use mem_cards::client::Cli;
//use mem_cards::misc::now_datetime;
//use mem_cards::sql::{mQuery,mConnect};

fn main() {

    let cli: Cli= Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error {}", e);
        std::process::exit(1);

    }
}

