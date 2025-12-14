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
    c,
    l,
    w,
    m,
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

        let commands_option = match args[1] {
            "c" => Some(Commands::c),
            "l" => Some(Commands::l),
            "w" => Some(Commands::w),
            "m" => Some(Commands::m),
            _ => None,
        };

        let commands: Commands = match commands_option {
            Some(commands) => commands,
            None => Err("Invalid color"),
        };

        let filename = args[2].clone();
        Ok(Config { commands, filename })
    }
}

fn logic(config: Config) {
    match config.commands {
        Commands::c => {
            println!("Calculating the number of bytes in the {}", config.filename);
            print_bytes(&config.filename);
        }
        Commands::l => {
            println!("Calculating the number of lines in the {}", config.filename);
            print_lines(&config.filename);
        }
        Commands::w => {
            println!("Calculating the number of words in the {}", config.filename);
            print_words(&config.filename);
        }
        Commands::m => {
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
