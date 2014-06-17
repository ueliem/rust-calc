enum ParseStates {
    StartState,
    NumberState,
    PlusState,
    MinusState,
    StarState,
    SlashState,
    LParenState,
    RParenState
}
pub fn tokenize<'a>(tokenstring: &'a str) -> Option<Vec<::token::Token<'a>>> {
    let mut current_state = StartState;
    let mut current_token_start: uint = 0;
    let mut current_token_end: uint = 0;
    let mut all_tokens = Vec::new();

    for i in range(0, tokenstring.len()) {
        match current_state {
            StartState => {
                match tokenstring[i] {
                    48_u8..57_u8 => {//0-9 zero to nine
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    40_u8 => {// ( LPAREN
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_start = i;
                        current_state = RParenState;
                    },
                    _ => { /* ignore everything else */ }
                }
            },
            NumberState => {
                match tokenstring[i] {
                    // 48_u8..57_u8 => {//0-9 zero to nine
                    //     current_state = NumberState;
                    // },
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
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::NUMBER});
                        current_token_start = i;
                        current_state = RParenState;
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
                    48_u8..57_u8 => {//0-9 zero to nine
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
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::PLUS});
                        current_token_start = i;
                        current_state = RParenState;
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
                    48_u8..57_u8 => {//0-9 zero to nine
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
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::MINUS});
                        current_token_start = i;
                        current_state = RParenState;
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
                    48_u8..57_u8 => {//0-9 zero to nine
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
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::STAR});
                        current_token_start = i;
                        current_state = RParenState;
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
                    48_u8..57_u8 => {//0-9 zero to nine
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
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = RParenState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::SLASH});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            LParenState => {
                match tokenstring[i] {
                    48_u8..57_u8 => {//0-9 zero to nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = RParenState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::LPAREN});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            },
            RParenState => {
                match tokenstring[i] {
                    48_u8..57_u8 => {//0-9 zero to nine
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = NumberState;
                    },
                    43_u8 => {// + Plus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = PlusState;
                    },
                    45_u8 => {// - Minus
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = MinusState;
                    },
                    42_u8 => {// * Star
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = StarState;
                    },
                    47_u8 => {// / Slash
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = SlashState;
                    },
                    40_u8 => {// ( LPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = LParenState;
                    },
                    41_u8 => {// ( RPAREN
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = RParenState;
                    },
                    _ => {
                        current_token_end = i;
                        all_tokens.push(::token::Token{value: tokenstring.slice(current_token_start,current_token_end), toktype: ::token::RPAREN});
                        current_token_start = i;
                        current_state = StartState;
                    }
                }
            }
        }
    }
    return Some(all_tokens);
}
