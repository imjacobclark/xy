use std::env;
use std::fs;

mod scanner;
mod ast;
mod common;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let tokens = scanner::scan(fs::read_to_string::<&String>, file_path);

    let partitioned: Vec<Vec<scanner::Token>> = common::split_vector_by_type::<scanner::Token>(tokens, scanner::Token::Return);

    println!("{:?}", partitioned);
}
