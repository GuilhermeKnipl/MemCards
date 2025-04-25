use clap::Parser;
use mem_cards::client::Cli;
use mem_cards::database;

fn main() {

    let conn = database::establish_connection();
    //conn    

    let cli: Cli= Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error {}", e);
        std::process::exit(1);
    
    }

