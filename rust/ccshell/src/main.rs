use std::io::{self, Write};

fn main() {
    let commads = ["exit", "echo"];
    loop {
        let mut position_counter: usize = 0;
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
            for stuff in input {
                position_counter += 1;
                if position_counter == 1 {
                    continue;
                }
                print!("{} ", stuff);
            }
            println!("");
        } else if input[0] == "type" {
            if commads.contains(&input[1]) {
                println!("{} is a shell builtin", input[1]);
            } else {
                println!("{}: not found", input[1]);
            }
        } else {
            println!("{}: command not found", input[0]);
        }
    }
}
