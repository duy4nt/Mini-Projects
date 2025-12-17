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
    run(&content);
}

fn run_prompt() {
    loop {
        let mut buffer: String = String::new();
        print!("> ");
        io::stdin().read_line(&mut buffer).expect("Failed to read line. pLeas einput again");
        if buffer == "" {
            break;
        }
        run(&buffer);
    }
}

fn run(source : &String) {
    let lex: Lexer = Lexer::new();
    let tokens: Vec<tokens> = lex.scanTokens();

    for  token in tokens {
        println!("{:?}", token);
    }
}

// TODO: Lexer
fn lexer() {
    
}
// TODO: Parser

enum token_type {
    None,
    Bool,
    Number,
    String,
}

enum token {
    type: token_type,
    lexeme: String,
    literal: String,
    line: i32,
}

impl token {
    fn new(type: token_type, lexeme: String, literal: String, line: i32) -> &self {
        self.type = type,
        self.lexeme = lexeme,
        self.lteral = literal,
        self.line = line,
    }

    fn to_string() -> &String {
        return (type + " " + lexeme + " "+ literal);
    }
}

fn is_at_end(source : &String, current : usize) -> bool {
    current > source.len()
}
