use html_parser_rs::Lexer;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./examples/index.html").expect("Failed to read file.");

    let mut lexer = Lexer::new(contents);

    for token in lexer.parse() {
        println!("{token:?}");
    }
}
