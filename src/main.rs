use clap::Parser;
use mem_cards::client::Cli;


fn main() {
    let cli: Cli= Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error {}", e);
        std::process::exit(1);
    
    }
}
