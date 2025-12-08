use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::process::Command;
use std::{env, fs};

fn main() {
    let commads = ["exit", "echo"];
    'outer: loop {
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
                let env_path = env::var_os("PATH").expect("Failed to get the PATH value");
                for dir in env::split_paths(&env_path) {
                    let path = dir.join(input[1]);
                    if let Ok(metadata) = fs::metadata(&path) {
                        if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                            println!("{}", path.display());
                            continue 'outer;
                        }
                    }
                }
                println!("{}: not found", input[1]);
            }
        } else {
            let env_path = env::var_os("PATH").expect("Failed to get the PATH value");
            for dir in env::split_paths(&env_path) {
                let path = dir.join(input[0]);
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                        let output = Command::new(input[0])
                            .args(&input[1..])
                            .output()
                            .expect("Failed to execut the file");
                        print!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
                        continue 'outer;
                    }
                }
            }
            println!("{}: command not found", input[0]);
        }
    }
}
