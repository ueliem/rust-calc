// use std::io::BufferedReader;
// use std::io::println;
// use std::io;
mod token;
enum ParseStates {
    StartState
}

fn main() {
    // let mut reader = BufferedReader::new(io::stdin());
    // let input = reader.read_line().unwrap();
    // println!("Hello World!");
    // println!("{}", &input);
    let program = "1 + 1";
    // let mut accumulator = 0;
    let current_state = StartState;
    // let mut current_token_value: std::string::String;
    let mut all_tokens = Vec::new();

    for c in program.chars() {
        match current_state {
            StartState => {
                match c {
                    '0' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '1' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '2' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '3' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '4' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '5' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '6' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '7' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '8' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '9' => {
                        println!("NUMBER");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::NUMBER});
                    },
                    '+' => {
                        println!("PLUS");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::PLUS});
                    },
                    '-' => {
                        println!("MINUS");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::MINUS});
                    },
                    '*' => {
                        println!("STAR");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::STAR});
                    },
                    '/' => {
                        println!("SLASH");
                        all_tokens.push(token::Token{value: std::str::from_char(c), toktype: token::SLASH});
                    },
                    _ => { /* ignore everything else */ }
                }
            }
        }
    }
    // println!("{}", accumulator);
    println!("{}", all_tokens);
}
