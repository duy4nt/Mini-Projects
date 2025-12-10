use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::process::Command;
use std::{env, fs};

fn main() {
    let commads = ["exit", "echo", "type", "pwd", "cd"];
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
        } else if input[0] == "pwd" {
            let current_dir = env::current_dir().expect("Failed to get the current directory");
            println!("{}", current_dir.display());
        } else if input[0] == "cd" {
            let mut current_dir = env::current_dir().expect("Failed to get the current directory");
            if input[1]
                .chars()
                .nth(0)
                .expect("Failed to get the first element")
                == '/'
            {
                let ffs = match env::set_current_dir(input[1]) {
                    Ok(p) => p,
                    Err(error) => println!("{}: {}: No such file or directory", input[0], input[1]),
                };
                current_dir = input[1].into();
            }
            if input[1]
                .chars()
                .nth(0)
                .expect("Faield to get the first element")
                == '.'
                && input[1]
                    .chars()
                    .nth(1)
                    .expect("Faield to get the second element")
                    == '/'
            {
                let to_add = input[1];
                // Error: still doesn't have a size known at compile-time
                current_dir = current_dir.join(to_add[2..]);
                let ffs = match env::set_current_dir(current_dir) {
                    Ok(p) => p,
                    Err(error) => println!("No such file or directory"),
                };
            }
            if input[1]
                .chars()
                .nth(0)
                .expect("Failed to get the first element")
                == '.'
                && input[1]
                    .chars()
                    .nth(1)
                    .expect("Failed to get the second element")
                    == '.'
            {
                //TODO: move to parent dir
            }
            if input[1]
                .chars()
                .nth(1)
                .expect("Failed to get the first element")
                == '~'
            {
                current_dir = "/home".into();
                env::set_current_dir("/home").is_ok();
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
                        print!(
                            "{}",
                            String::from_utf8_lossy(&output.stdout).trim_end_matches("\n")
                        );
                        continue 'outer;
                    }
                }
            }
            println!("{}: command not found", input[0]);
        }
    }
}
