use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    // format!("Hello, {name}!")
    let my_str = include_str!("kk_th.txt");
    // let path = Path::new("word_th.txt").exists();
    // format!("{name}{path}")
    format!("{my_str}")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_token(text: String) -> Vec<String> {
    
    let tokenizer = NewmmTokenizer::new("words_th.txt");
    let tokens = tokenizer.segment(&text, true, false).unwrap();
    let token_strings: Vec<String> = tokens.iter().cloned().collect();

    // Print tokens
    for token in &token_strings {
        println!("{}", token);
    }

    token_strings
}