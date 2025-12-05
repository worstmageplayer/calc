pub mod token;
use crate::lexer::token::Token;

pub fn tokenize(input: String) -> Option<Vec<Token>> {
    let mut i: usize = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while i < input_chars.len() {
        let mut c = input_chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() || c == '.' {
            let start = i;
            let mut dot: bool = false;
            let mut dotpos = 0;

            while i < input_chars.len() {
                c = input_chars[i];

                if c.is_ascii_digit() {
                    i += 1;
                } else if c == '.' {
                    if dot { return None }
                    dot = true;
                    dotpos = i;
                    i += 1;
                } else {
                    break;
                }
            }

            // Change this
            // erm, 10 ^ pos of char - pos of dot or somethign

            let number: String = input_chars[start..i].iter().collect();
        }
    }

    let b = &input;

    if b.len() > 4 {
        let a: Vec<Token> = vec![Token::SemiColon];
        Some(a)
    } else {
        let a: Vec<Token> = vec![Token::End];
        Some(a)
    }
}
