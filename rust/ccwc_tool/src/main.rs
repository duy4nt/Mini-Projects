use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let filename = &args[2].to_string();

    let commands: [&str; 4] = ["c", "l", "w", "m"];
    
    println!("{:?}", commands);

    if query == commands[2] {
        println!("It workssssssssss");
    }

}

fn read_file() {

}

fn print_bytes() {

}

fn print_lines() {

}

fn print_words() {

}

fn print_characters() {

}
