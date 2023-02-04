use std::io::{self, Error};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Number(char),
    Letter(char),
    Symbol(char),
    Return,
    Eq,
    Add,
    Subtract,
    Divide,
    Multiply,
}

pub fn strip_whitespace(tokens: Vec<String>) -> Vec<String> {
    return tokens
        .into_iter()
        .filter(|x| x != "")
        .collect();
}

pub fn translate_input_to_chars_vec(untokenised_input: &String) -> Vec<String> {
    return untokenised_input
        .chars()
        .map(|c| c.to_string())
        .collect();
}

fn tokenise(s: String) -> Token {
    let c = s.chars().next().unwrap();

    match c {
        c if c == '+' => Token::Add,
        c if c == '-' => Token::Subtract,
        c if c == '/' => Token::Divide,
        c if c == '*' => Token::Multiply,
        c if c == '=' => Token::Eq,
        c if c == '\n' => Token::Return,
        c if c.is_numeric() => Token::Number(c),
        c if c.is_alphabetic() => Token::Letter(c),
        _ => Token::Symbol(c),
    }
}

pub fn scan<P>(reader: fn(P) -> Result<String, Error>, file_path: P) -> Vec<Token> {
    let input = reader(file_path).unwrap_or_else(|_| {
        eprintln!("Error reading file");
        std::process::exit(1);
    });

    let untokenised_input_vec = translate_input_to_chars_vec(&input);
    let clean_untokenised_input_vec: Vec<String> = strip_whitespace(untokenised_input_vec);

    let tokenised_input: Vec<Token> = clean_untokenised_input_vec
        .iter()
        .map(|x| tokenise(x.to_string()))
        .collect();

    return tokenised_input;
}
