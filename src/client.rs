use clap::{Parser, Subcommand};
use crate::commands;

#[derive(Parser)]
#[command(name = "mem")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Greet {
        name: String,
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    Add {
    
    }

}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Greet { name, count } => {
                commands::greet::run(name, *count);
            }
            Commands::Add { } => {
                commands::add_word::run();
            }
        }
        Ok(())
    }
}

