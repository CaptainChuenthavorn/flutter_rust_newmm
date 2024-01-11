# flutter_rust_newmm

flutter_rust_newmm is a project that integrates Rust within Flutter to utilize the Newmm tokenizer for Thai words. This integration is achieved through the use of the `flutter_rust_bridge` repository.

## Integration with flutter_rust_bridge

To get started, follow the quickstart guide in the [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) repository:

```bash
cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0' && \
    flutter_rust_bridge_codegen create my_app && cd my_app && flutter run
```

After this process, you'll have a Flutter app with Rust code integrated.
Ps. We need to execute the code generator whenever the Rust code is changed, or use --watch to automatically re-generate when code changes:
```bash
flutter_rust_bridge_codegen generate --watch
```
## Adding Newmm Tokenizer
1. Add the following dependencies to your Cargo.toml:
```toml
[dependencies]
flutter_rust_bridge = "=2.0.0-dev.15"
nlpo3 = "1.3.2"
```
2. Create a function in rust/src/api/simple.rs:
```rust
#[flutter_rust_bridge::frb(sync)]
pub fn get_token(text: String) -> Vec<String> {
    let my_str = include_str!("words_th.txt");

    // Assuming words are separated by a delimiter (e.g., newline)
    let delimiter = "\n";
    let split_words: Vec<&str> = my_str.split(delimiter).collect();

    // Convert split words into Vec<String>
    let words_split: Vec<String> = split_words.iter().map(|&s| s.to_string()).collect();

    // Create NewmmTokenizer from the word list
    let tokenizer = NewmmTokenizer::from_word_list(words_split);
    
    // Segment the text and retrieve tokens
    let tokens = tokenizer.segment(&text, true, false).unwrap();
    let token_strings: Vec<String> = tokens.iter().cloned().collect();

    // Print tokens (optional)
    for token in &token_strings {
        println!("{}", token);
    }

    token_strings
}

```
This modification resolves issues related to tokenizer loading the dictionary path.
3. Testing
In your Flutter app (lib/main.dart), you can run:
```dart
Future<List<String>> fetchData() async {
  final response = await Future<List<String>>.delayed(
    const Duration(seconds: 0),
    () => getToken(text: 'ลงน้ำหนักเท้าไม่เท่ากัน'),
  );
  print(response);
  return response;
}

```
the result You will get
![Screenshot 2567-01-12 at 01 18 06](https://github.com/CaptainChuenthavorn/flutter_rust_newmm/assets/38762000/8be7db3a-c1ff-44bf-a8c8-a2b5edb09506)

Feel free to modify the provided code snippets according to your specific needs.
