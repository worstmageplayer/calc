#![allow(dead_code)]

mod parser;
mod lexer;
mod evaluator;
mod number;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No arguments passed")
    }

    lexer::tokenize("aaaa".to_string());

    let input: String = args[1..].join("");
    let input_without_whitespaces: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    println!("{}", input_without_whitespaces);
}
