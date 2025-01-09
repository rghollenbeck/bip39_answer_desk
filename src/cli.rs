// src/cli.rs
use clap::{Parser, Subcommand};
// use std::fs;

// CLI definition
#[derive(Parser)]
#[command(name = "bip39_answer_desk", about = "A CLI tool for BIP39 word manipulation")]
pub struct Cli {
    #[command(subcommand)]
   pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert binary to word
    Bw { binary: String },
    /// Convert decimal to word
    Dw { decimal: u32 },
    /// Convert word to binary
    Wb { word: String },
    /// Convert word to decimal
    Wd { word: String },
}
