mod cli; // Import the CLI module

//use bip39_answer_desk::bip39::decimal_to_word;
use clap::Parser;
//use bip39_answer_desk::bip39::{binary_to_word, load_wordlist};
use bip39_answer_desk::bip39::{binary_to_word, decimal_to_word, word_to_binary, word_to_decimal, load_wordlist};
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

        Commands::Dw { decimal } => match decimal_to_word(&word_list, decimal) {
            Some(word) => println!("Word: {}", word),
            None => println!("Decimal value not found"),
        },

        Commands::Wb { word } => match word_to_binary(&word_list, &word) {
            Some(binary) => println!("Binary: {}", binary),
            None => println!("Error: Word not found"),
        },

        Commands::Wd { word } => match word_to_decimal(&word_list, &word) {
            Some(decimal) => println!("Decimal: {}", decimal),
            None => println!("Error: Word not found"),
        },
    }
}

