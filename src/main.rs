use std::env;
use std::fs;

mod scanner;
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{:?}", scanner::scan(fs::read_to_string::<&String>, file_path));
}
