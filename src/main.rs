mod cli; // Import the CLI module

use clap::Parser;
use bip39_answer_desk::bip39::{binary_to_word, load_wordlist};
use cli::{Cli, Commands}; // Use the Cli and Commands from cli.rs

fn main() {
    let cli = Cli::parse(); // Parse the CLI arguments

    // Load the word list with error handling
    let word_list = match load_wordlist("src/bip39list.json") {
        Ok(word_list) => word_list,
        Err(err) => {
            eprintln!("Error loading word list: {}", err);
            return;
        }
    };

    // Match and handle CLI commands
    match cli.command {
        Commands::Bw { binary } => match binary_to_word(&word_list, &binary) {
            Some(word) => println!("Word: {}", word),
            None => println!("Binary value not found"),
        },
        Commands::Dw { decimal } => {
            println!("Decimal to word not implemented yet: {}", decimal);
        }
        Commands::Wb { word } => {
            println!("Word to binary not implemented yet: {}", word);
        }
        Commands::Wd { word } => {
            println!("Word to decimal not implemented yet: {}", word);
        }
    }
}

