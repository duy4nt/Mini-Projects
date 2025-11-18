use std::fs;

fn main() {
    let content =
        fs::read_to_string("valid.json").expect("Failed to read the contents of the file");
    println!("{:?}", content);
    let content: Vec<_> = content.trim().as_bytes().collect();
}

// TODO: Lexer
// TODO: Parser

enum json_values {
    Null,
    Bool,
    Number,
    String,
}
