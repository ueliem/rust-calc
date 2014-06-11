enum ParseStates {
    StartState,
    NumberState,
    PlusState,
    MinusState,
    StarState,
    SlashState
}
pub fn tokenize<'a>(tokenstring: &'a str) -> Vec<::token::Token<'a>> {
    let mut current_state = StartState;
    let mut current_token_start: uint = 0;
    let mut current_token_end: uint = 0;
    let mut all_tokens = Vec::new();

    for i in range(0, tokenstring.len()) {
        match current_state {
            StartState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    49_u8 => {// 1 One
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    50_u8 => {// 2 Two
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    51_u8 => {// 3 Three
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    52_u8 => {// 4 Four
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    53_u8 => {// 5 Five
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    54_u8 => {// 6 Six
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    55_u8 => {// 7 Seven
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    56_u8 => {// 8 Eight
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    57_u8 => {// 9 Nine
                        current_token_start = i;
                        current_state = NumberState;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(i,i+1), toktype: ::token::NUMBER});
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        // all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => { /* ignore everything else */ }
                }
            },
            NumberState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                    },
                    49_u8 => {// 1 One
                    },
                    50_u8 => {// 2 Two
                    },
                    51_u8 => {// 3 Three
                    },
                    52_u8 => {// 4 Four
                    },
                    53_u8 => {// 5 Five
                    },
                    54_u8 => {// 6 Six
                    },
                    55_u8 => {// 7 Seven
                    },
                    56_u8 => {// 8 Eight
                    },
                    57_u8 => {// 9 Nine
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            PlusState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    49_u8 => {// 1 One
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    50_u8 => {// 2 Two
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    51_u8 => {// 3 Three
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    52_u8 => {// 4 Four
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    53_u8 => {// 5 Five
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    54_u8 => {// 6 Six
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    55_u8 => {// 7 Seven
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    56_u8 => {// 8 Eight
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    57_u8 => {// 9 Nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            MinusState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    49_u8 => {// 1 One
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    50_u8 => {// 2 Two
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    51_u8 => {// 3 Three
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    52_u8 => {// 4 Four
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    53_u8 => {// 5 Five
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    54_u8 => {// 6 Six
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    55_u8 => {// 7 Seven
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    56_u8 => {// 8 Eight
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    57_u8 => {// 9 Nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            StarState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    49_u8 => {// 1 One
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    50_u8 => {// 2 Two
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    51_u8 => {// 3 Three
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    52_u8 => {// 4 Four
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    53_u8 => {// 5 Five
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    54_u8 => {// 6 Six
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    55_u8 => {// 7 Seven
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    56_u8 => {// 8 Eight
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    57_u8 => {// 9 Nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            SlashState => {
                match tokenstring[i] {
                    48_u8 => {// 0 Zero
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    49_u8 => {// 1 One
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    50_u8 => {// 2 Two
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    51_u8 => {// 3 Three
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    52_u8 => {// 4 Four
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    53_u8 => {// 5 Five
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    54_u8 => {// 6 Six
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    55_u8 => {// 7 Seven
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    56_u8 => {// 8 Eight
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    57_u8 => {// 9 Nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            }
        }
    }
    return all_tokens;
}
