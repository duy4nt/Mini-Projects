use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

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
    let mut file_object = match file_object_result {
        Ok(file) => file,
        Err(error) => panic!("Problem: {}", error),
    };
    let mut content = String::new();
    file_object
        .read_to_string(&mut content)
        .expect("Failed parsing the file");
    run(&mut content);
}

fn run_prompt() {
    loop {
        let mut buffer: String = String::new();
        print!("> ");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line. pLeas einput again");
        if buffer == "" {
            break;
        }
        run(&mut buffer);
    }
}

fn run(source: &mut String) {
    let mut lex: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lex.scanTokens();

    for token in tokens {
        println!("{}", token);
    }
}

#[derive(Debug)]
enum TokenType {
    // Single character tokens
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    // Literals
    String,
    Number,
    //End of File
    EOF,
}

struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Lexer {
    fn new(source: &mut String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scanTokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            String::new(),
            self.line,
        ));

        std::mem::take(&mut self.tokens)
    }

    fn scanTokens() {
        let c: char = self.advance();

        match c {
            '{' => self.addToken(TokenType::LeftBrace, String::new()),
            '}' => self.addToken(TokenType::RightBrace, String::new()),
            ',' => self.addToken(TokenType::Comma, String::new()),
            ':' => self.addToken(TokenType::Colon, String::new()),
            '"' => self.string(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c if c.is_ascii_digit() || c == '-' => self.number,
            _ => println!("Unexpexted error"),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.cuurent];
        self.current += 1;
        c
    }

    fn string(&mut self) {
        //TODO
    }

    fn number(&mut self) {
        //TODO
    }

    fn addToken(&mut self, token_type: TokenType, literal: String) {
        //TODO
    }

    fn is_at_end(&mut self) -> bool {
        self.current > self.source.len()
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
