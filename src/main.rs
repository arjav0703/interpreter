use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                detect_parentheses(&file_contents);
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn detect_parentheses(input: &str) {
    let input_ver: Vec<char> = input.chars().collect();
    for i in &input_ver {
        match i {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            _ => (),
        }
    }
    print!("EOF  null");
    // print!("{:?}", input_ver)
}
