// src/lib.rs
use serde::Deserialize;

pub mod utils; // Expose the utils module
pub mod bip39; // Expose the bip39 module
pub mod cli;   // Expose the cli module (if you want to use CLI logic in tests or other binaries)

pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.binary == binary).map(|w| w.word.as_str())
}

pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.word == word).map(|w| w.binary.as_str())
}

#[derive(Deserialize)]
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

#[derive(Deserialize)]
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

pub fn load_wordlist(path: &str) -> WordList {
    let json_data = std::fs::read_to_string(path).expect("Failed to read bip39list.json");
    serde_json::from_str(&json_data).expect("Failed to parse bip39list.json")
}


pub fn decimal_to_word(word_list: &WordList, decimal: u32) -> Option<&str> {
    let binary = format!("{:011b}", decimal);
    binary_to_word(word_list, &binary)
}
	
pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<u32> {
    word_to_binary(word_list, word).and_then(|binary| u32::from_str_radix(binary, 2).ok())
}
