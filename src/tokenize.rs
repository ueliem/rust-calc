enum ParseStates {
    StartState
}
pub fn tokenize<'a>(tokenstring: &'a str) -> Vec<::token::Token<'a>> {
    let current_state = StartState;
    let mut current_token_start: int;
    let mut current_token_end: int;
    let mut all_tokens = Vec::new();

    for i in range(0, tokenstring.len()) {
        match current_state {
            StartState => {
                match tokenstring[i] {
                    48_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    49_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    50_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    51_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    52_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    53_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    54_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    55_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    56_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    57_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    43_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::PLUS});
                    },
                    45_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::MINUS});
                    },
                    42_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::STAR});
                    },
                    47_u8 => {
                        all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::SLASH});
                    },
                    _ => { /* ignore everything else */ }
                }
            }
        }
    }
    return all_tokens;
}
