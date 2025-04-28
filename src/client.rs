use std::u8;

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
    Stats {
        name: String,
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    Add {
    
    },
    Search{
        word: String,

        #[arg(short = 's', long = "sort-asc" ,default_value_t = false, help = "Sorting Ascending (default = Desc)")]
        sort_asc: bool,
        
        #[arg(short = 'l', long = "limit" ,default_value_t = 10, help = "Limit results")]
        limit: u8,

        #[arg(short = 'a', long = "all" ,default_value_t = false, help= "List all records")]
        records: bool,

        #[arg(short = 'd', long = "detail" ,default_value_t = false, help= "Show Word Info")]
        detail: bool
    },

    Backup {
        end_path: String
    }

}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Stats { name, count } => {
                commands::stats::run(name, *count);
            }
            Commands::Add { } => {
                commands::add_word::run();
            }
            Commands::Search { word, sort_asc, limit, records, detail} => {
                commands::search_word::run(word, sort_asc, limit, records, detail);
            }
            Commands::Backup { end_path } => {
                commands::backup::run(end_path);
            }
        }
        Ok(())
    }
}

