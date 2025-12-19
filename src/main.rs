use std::env;
use std::fs;
use std::process::exit;

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
                let exitcode = detect_parentheses(&file_contents);
                exit(exitcode);
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn detect_parentheses(input: &str) -> i32 {
    let mut exit_code = 0;
    let input_ver: Vec<char> = input.chars().collect();
    let mut line_number = 1;
    for i in &input_ver {
        if *i == '\n' {
            line_number += 1;
        } else if let Some(lexeme) = LexemeType::from_char(*i) {
            lexeme.print();
        } else {
            eprintln!("[line {line_number}] Error: Unexpected character: {i}");
            exit_code = 65;
        }
    }
    print!("EOF  null");
    exit_code
}

enum LexemeType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    STAR,
    EOF,
    DOT,
    COMMA,
    PLUS,
    SEMICOLON,
    SLASH,
    MINUS,
}

impl LexemeType {
    fn to_string(&self) -> &str {
        match self {
            LexemeType::LEFT_PAREN => "LEFT_PAREN ( null",
            LexemeType::RIGHT_PAREN => "RIGHT_PAREN ) null",
            LexemeType::LEFT_BRACE => "LEFT_BRACE { null",
            LexemeType::RIGHT_BRACE => "RIGHT_BRACE } null",
            LexemeType::STAR => "STAR * null",
            LexemeType::EOF => "EOF  null",
            LexemeType::DOT => "DOT . null",
            LexemeType::COMMA => "COMMA , null",
            LexemeType::PLUS => "PLUS + null",
            LexemeType::SEMICOLON => "SEMICOLON ; null",
            LexemeType::SLASH => "SLASH / null",
            LexemeType::MINUS => "MINUS - null",
        }
    }

    fn from_char(c: char) -> Option<LexemeType> {
        match c {
            '(' => Some(LexemeType::LEFT_PAREN),
            ')' => Some(LexemeType::RIGHT_PAREN),
            '{' => Some(LexemeType::LEFT_BRACE),
            '}' => Some(LexemeType::RIGHT_BRACE),
            '*' => Some(LexemeType::STAR),
            '.' => Some(LexemeType::DOT),
            ',' => Some(LexemeType::COMMA),
            '+' => Some(LexemeType::PLUS),
            ';' => Some(LexemeType::SEMICOLON),
            '-' => Some(LexemeType::MINUS),
            '/' => Some(LexemeType::SLASH),
            _ => None,
        }
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }
}
