use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");
        let mut input: Vec<&str> = input.trim().split_whitespace().collect();

        if input[0] == "exit" {
            break;
        } else if input[0] == "echo" {
            println!("{:?} some", input);
        } else {
            println!("{}: command not found", input[0]);
        }
    }
}

