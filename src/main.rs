// use std::io::BufferedReader;
// use std::io::println;
// use std::io;

enum ParseStates {
    StartState
}

enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    STAR,
    SLASH
}

struct Token {
    value: std::string::String,
    toktype: TokenType
}

impl std::fmt::Show for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
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
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '1' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '2' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '3' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '4' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '5' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '6' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '7' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '8' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '9' => {
                        println!("NUMBER");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: NUMBER});
                    },
                    '+' => {
                        println!("PLUS");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: PLUS});
                    },
                    '-' => {
                        println!("MINUS");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: MINUS});
                    },
                    '*' => {
                        println!("STAR");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: STAR});
                    },
                    '/' => {
                        println!("SLASH");
                        all_tokens.push(Token{value: std::str::from_char(c), toktype: SLASH});
                    },
                    _ => { /* ignore everything else */ }
                }
            }
        }
    }
    // println!("{}", accumulator);
    println!("{}", all_tokens);
}
