use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    logic(config);
}

enum Commands {
    C,
    L,
    W,
    M,
}

struct Config {
    commands: Commands,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let commands_option = match args[1].as_str() {
            "c" => Some(Commands::C),
            "l" => Some(Commands::L),
            "w" => Some(Commands::W),
            "m" => Some(Commands::M),
            _ => None,
        };

        let commands: Commands = match commands_option {
            Some(commands) => commands,
            None => return Err("Invalid command"),
        };

        let filename = args[2].clone();
        Ok(Config { commands, filename })
    }
}

fn logic(config: Config) {
    match config.commands {
        Commands::C => {
            println!("Calculating the number of bytes in the {}", config.filename);
            print_bytes(&config.filename);
        }
        Commands::L => {
            println!("Calculating the number of lines in the {}", config.filename);
            print_lines(&config.filename);
        }
        Commands::W => {
            println!("Calculating the number of words in the {}", config.filename);
            print_words(&config.filename);
        }
        Commands::M => {
            println!(
                "Calcukating the number of characters in the {}",
                config.filename
            );
            print_characters(&config.filename);
        }
    }
}

fn print_bytes(filename: &str) {
    println!("Reading the content from the filename {:?}", filename);
    let content = fs::read_to_string(filename).expect("Failed to read the line");
    // println!("{content}");
    // println!("{:?}", filename);
    println!(
        "The number of bytes in the file are {}",
        content.as_bytes().iter().count()
    );
}

fn print_lines(filename: &str) {
    println!("Reading the content from the filename {:?}", filename);
    let content = fs::read_to_string(filename).expect("Failed to read the line");
    // println!("{content}");
    // println!("{:?}", filename);
    println!(
        "The number of lines in the file are {}",
        content.lines().count()
    );
}

fn print_words(filename: &str) {
    println!("Reading the content from the filename {:?}", filename);
    let content = fs::read_to_string(filename).expect("Failed to read the line");
    // println!("{content}");
    // println!("{:?}", filename);
    println!(
        "The number of words in the file are {}",
        content.split_whitespace().count()
    );
}

fn print_characters(filename: &str) {
    println!("Reading the content from the filename {:?}", filename);
    let content = fs::read_to_string(filename).expect("Failed to read the line");
    // println!("{content}");
    // println!("{:?}", filename);
    println!(
        "The number of character in the file are {}",
        content.chars().count()
    );
}
