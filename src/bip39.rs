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

