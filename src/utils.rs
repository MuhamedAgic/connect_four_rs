
use std::io::{stdin, stdout, Write};

pub fn get_cli_input() -> String {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

pub fn cli_confirms() -> bool {
    loop {
        let input = get_cli_input().trim().to_lowercase();
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}


