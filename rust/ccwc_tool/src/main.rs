use std::env;
use std::fs;
use std::process;
use std::result;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    logic(config);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn logic(config: Config) {
    let commands: [&str; 4] = ["c", "l", "w", "m"];

    if config.query == commands[0] {
        println!("Calculating the number of bytes in the {}", config.filename);
        print_bytes(&config.filename);
    } else if config.query == commands[1] {
        println!("Calculating the number of lines in the {}", config.filename);
        print_bytes(&config.filename);
    } else if config.query == commands[2] {
        println!("Calculating the number of words in the {}", config.filename);
        print_bytes(&config.filename);
    } else if config.query == commands[3] {
        println!(
            "Calculating the number of characters in the {}",
            config.filename
        );
        print_bytes(&config.filename);
    } else {
        println!("Damn! That shi is not implemented.");
    }
}

fn print_bytes(filename: &str) {
    println!("{:?}", filename);
}

fn print_lines(filename: &str) {
    println!("{:?}", filename);
}

fn print_words(filename: &str) {
    println!("{:?}", filename);
}

fn print_characters(filename: &str) {
    println!("{:?}", filename);
}
