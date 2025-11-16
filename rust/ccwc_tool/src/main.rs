use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let filename = &args[2].to_string();

    let byte_command: &str = "-w";
    
    println!("{byte_command}");

    match query {
         byte_command => {
            println!("Calculating the number of words in the file"),
        }
        _ => {
            println!("Damn! That's not implemented"),
        }
    
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
