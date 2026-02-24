use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path;
use std::collections::HashMap;

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
    run(&mut content);
}

fn run_prompt() {
    loop {
        let mut buffer: String = String::new();
        print!("> ");
        io::stdin().read_line(&mut buffer).expect("Failed to read line. pLeas einput again");
        if buffer == "" {
            break;
        }
        run(&mut buffer);
    }
}

fn run(source : &mut String) {
    let lex: Lexer = Lexer::new(&mut source);
    let tokens: Vec<tokens> = lex.scanTokens();

    for  token in tokens {
        println!("{}", token);
    }
}

enum TokenType {
    // Single character tokens
    LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, COLON,

    STRING, NUMBER,

    EOF,
}

struct Lexer {
    source: String,
    tokens: HasMap
}

impl Lexer {
    fn new(source: &mut String) {
        //TODO
    }

    fn scanTokens() {
        //TODO
    }
}

struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Token {
    fn new(tokentype: TokenType, lexeme: String, literal: String, line: i32) -> &self {
        self.tokentype = tokentype,
        self.lexeme = lexeme,
        self.lteral = literal,
        self.line = line,
    }

    fn to_string() -> &String {
        return tokentype + " " + lexeme + " "+ literal;
    }
}

fn is_at_end(source : &String, current : usize) -> bool {
    current > source.len()
}
