use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        panic!("Problem: Excessive arguments");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
    let file_object = File::open("valid.json");
    let content = match file_object {
        Ok(content) => content,
        Err(error) => panic!("Problem: {}", error),
    };
    println!("{:?}", content);
}

fn run_file(path_string: &String) {
    //TODO
    let path_from_string = Path::new(path_string);
    let file_object_result = File::open(path_string);
    let file_object = match file_object_result {
        Ok(file) => file,
        Err(error) => panic!("Problem: {}", error),
    };
    let content = String::new();
    file_object
        .read_to_string(&mut content)
        .expect("Failed parsing the file");
    run();
}

fn run_prompt() {
    //TODO
}

fn run() {}

// TODO: Lexer
// TODO: Parser

enum json_tokens {
    Null,
    Bool,
    Number,
    String,
}
