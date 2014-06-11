pub fn parse(tokens: Vec<::token::Token>) -> int {
    let mut accumulator = 0;
    for i in range(0, tokens.len()) {
            match tokens.get(i).toktype {
            ::token::MINUS => return parse_minus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
            _ => continue
        }
    }
    for i in range(0, tokens.len()) {
            match tokens.get(i).toktype {
            ::token::PLUS => return parse_plus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
            _ => continue
        }
    }
    for i in range(0, tokens.len()) {
            match tokens.get(i).toktype {
            ::token::SLASH => return parse_slash(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
            _ => continue
        }
    }
    for i in range(0, tokens.len()) {
        match tokens.get(i).toktype {
            ::token::STAR => return parse_star(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
            _ => continue
        }
    }
    return 5000;
}

pub fn getleft(tokens: &[::token::Token<>]) -> int {
    println!("{}", tokens);
    if (tokens.len() == 1 && tokens[0].is_terminal()) {
        match tokens[0].toktype {
            ::token::NUMBER => {
                println!("found terminal number {}", tokens[0]);
                return parse_number(tokens[0].value.to_owned());
            },
            _ => {  }
        }
    }
    else {
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::MINUS => return parse_minus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::PLUS => return parse_plus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::SLASH => return parse_slash(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
            match tokens[i].toktype {
                ::token::STAR => return parse_star(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        return 5000;
    }
    return 5000;
}

pub fn getright(tokens: &[::token::Token<>]) -> int {
    println!("{}", tokens);
    if (tokens.len() == 1 && tokens[0].is_terminal()) {
        match tokens[0].toktype {
            ::token::NUMBER => {
                println!("found terminal number {}", tokens[0]);
                return parse_number(tokens[0].value.to_owned());
            },
            _ => {  }
        }
    }
    else {
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::MINUS => return parse_minus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::PLUS => return parse_plus(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
                match tokens[i].toktype {
                ::token::SLASH => return parse_slash(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        for i in range(0, tokens.len()) {
            match tokens[i].toktype {
                ::token::STAR => return parse_star(getleft(tokens.slice_to(i)), getright(tokens.slice_from(i+1))),
                _ => continue
            }
        }
        return 5000;
    }
    return 5000;
}

pub fn parse_star(left: int, right: int) -> int {
    println!("Multiplying {} by {}", left, right);
    return left * right;
}

pub fn parse_slash(left: int, right: int) -> int {
    println!("Dividing {} by {}", left, right);
    return left / right;
}
pub fn parse_plus(left: int, right: int) -> int {
    println!("Adding {} and {}", left, right);
    return left + right;
}

pub fn parse_minus(left: int, right: int) -> int {
    println!("Subtarcting {} with {}", left, right);
    return left - right;
}

// pub fn parse_star(left: fn() -> int, right: fn() -> int) -> int {
//     return left() * right();
// }
//
// pub fn parse_slash(left: fn() -> int, right: fn() -> int) -> int {
//     return left() / right();
// }
// pub fn parse_plus(left: fn() -> int, right: fn() -> int) -> int {
//     return left() + right();
// }
//
// pub fn parse_minus(left: fn() -> int, right: fn() -> int) -> int {
//     return left() - right();
// }

pub fn parse_number(numstr: ::std::string::String) -> int {
    match from_str::<int>(numstr.as_slice()) {
        Some(0) => {println!("Some(0)"); return 0;},
        Some(n) => {println!("Some(n)"); return n;},
        None => {println!("None"); return -1;}
    }
}
