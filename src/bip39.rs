use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

#[derive(Deserialize)]
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

/// Load the BIP39 wordlist from a JSON file
pub fn load_wordlist(path: &str) -> Result<WordList, String> {
    let json_data = fs::read_to_string(path)
        .map_err(|_| format!("Failed to read the file at {}", path))?;
    serde_json::from_str(&json_data)
        .map_err(|_| "Failed to parse the JSON file".to_string())
}

/// Convert binary to a BIP39 word
pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.binary == binary).map(|w| w.word.as_str())
}

pub fn decimal_to_word(word_list: &WordList, decimal: u32) -> Option<&str> {
    if decimal > 2047 {
        return None; // Ensure input is within valid range
    }
    let binary = format!("{:011b}", decimal); // Convert decimal to 11-bit binary
    binary_to_word(word_list, &binary) // Reuse existing binary_to_word function
}

  pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list
        .wordlist
        .iter()
        .find(|entry| entry.word == word)
        .map(|entry| entry.binary.as_str())
}
      
      
pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<u32> {
    word_to_binary(word_list, word)
        .and_then(|binary| u32::from_str_radix(binary, 2).ok())
}


