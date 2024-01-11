use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;

use std::path::Path;

use std::fs::File;
use std::io::{self, Read};
#[flutter_rust_bridge::frb(sync)]
fn read_file(file_path: &str) -> Result<String, io::Error> {
    // Try to open the file
    let file_reader = File::open(file_path);

    // Check if the file was successfully opened
    let mut file = match file_reader {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    // Read the content of the file into a String
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(err) => Err(err),
    }
}
#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> Vec<String> {
    // format!("Hello, {name}!")
    // let my_str = include_str!("s/kk_th.txt");
    // let path = Path::new("word_th.txt").exists();
    // let path = Path::new("src/api/words_th.txt").exists();
    // format!("{name}{path}")
    let my_str = include_str!("words_th.txt");

    // Assuming your words are separated by some delimiter, like a newline character
    let delimiter = "\n";
    let split_words: Vec<&str> = my_str.split(delimiter).collect();

    // Convert the split words into Vec<String>
    let words: Vec<String> = split_words.iter().map(|&s| s.to_string()).collect();

    // Print the result or use it as needed
    words
    // format!("{my_str}")
}
    


#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_token(text: String) -> Vec<String> {

    // let tokenizer = NewmmTokenizer::new(r"words_th.txt");
    let my_str = include_str!("words_th.txt");

    // Assuming your words are separated by some delimiter, like a newline character
    let delimiter = "\n";
    let split_words: Vec<&str> = my_str.split(delimiter).collect();

    // Convert the split words into Vec<String>
    let words_split: Vec<String> = split_words.iter().map(|&s| s.to_string()).collect();

    // Print the result or use it as needed
    // words
    let words = words_split;
    let tokenizer = NewmmTokenizer::from_word_list(words);
    let tokens = tokenizer.segment(&text, true, false).unwrap();
    let token_strings: Vec<String> = tokens.iter().cloned().collect();

    // Print tokens
    for token in &token_strings {
        println!("{}", token);
    }

    token_strings
}