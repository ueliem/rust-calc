pub fn parse(tokens: &[::token::Token<>]) -> int {
    // println!("Tokens: {}", tokens);
    let count = tokens.len();
    // println!("count: {}", count);
    if tokens.len() == 1 {
        if tokens[0].is_terminal() {
            match tokens[0].toktype {
                ::token::NUMBER => {
                    //println!("found terminal number {}", tokens[0]);
                    return parse_number(tokens[0].value.to_string());
                },
                _ => {  }
            }
        }
        else {
            fail!("Reached leaf of parse tree and found non-terminal token.");
        }
    }
    for i in ::std::iter::range_step_inclusive((count as int)-1, 0, (-1)) {
        match tokens[(i as uint)].toktype {
            ::token::PLUS => {
                // println!("found plus {}", i);
                return parse_plus(getleft(tokens.slice_to((i as uint))), getright(tokens.slice_from((i as uint)+1)));
            },
            ::token::MINUS => {
                // println!("found minus {}", i);
                return parse_minus(getleft(tokens.slice_to((i as uint))), getright(tokens.slice_from((i as uint)+1)));
            },
            ::token::LPAREN => {
                // println!("found right initial RPAREN {}", i);
                fail!("Encountered unpaired LPAREN token.");
            },
            ::token::RPAREN => {
                // println!("found right initial RPAREN {}", i);
                break;
            },
            _ => continue
        }
    }
    for i in ::std::iter::range_step_inclusive((count as int)-1, 0, (-1)) {
        match tokens[(i as uint)].toktype {
            ::token::STAR => {
                // println!("found star {}", i);
                return parse_star(getleft(tokens.slice_to((i as uint))), getright(tokens.slice_from((i as uint)+1)));
            },
            ::token::SLASH => {
                // println!("found slash {}", i);
                return parse_slash(getleft(tokens.slice_to((i as uint))), getright(tokens.slice_from((i as uint)+1)));
            },
            ::token::LPAREN => {
                // println!("found right initial RPAREN {}", i);
                fail!("Encountered unpaired LPAREN token.");
            },
            ::token::RPAREN => {
                // println!("found right initial RPAREN {}", i);
                break;
            },
            _ => continue
        }
    }
    for i in ::std::iter::range_step_inclusive((count as int)-1, 0, (-1)) {
        match tokens[(i as uint)].toktype {
            ::token::RPAREN => {
                // println!("found right initial RPAREN {}", i);
                let mut parencount: uint = 1;
                for j in ::std::iter::range_step_inclusive(i-1, 0, (-1)) {
                    // println!("J {}", j);
                    match tokens[(j as uint)].toktype {
                        ::token::RPAREN => {
                            // println!("found RPAREN {}", (j as uint));
                            parencount += 1_u;
                            // println!("parencount is {}", parencount);
                        },
                        ::token::LPAREN => {
                            // println!("found LPAREN {}", (j as uint));
                            parencount -= 1_u;
                            // println!("parencount is {}", parencount);
                            //return parse(tokens.slice((j as uint), (i as uint)-1));
                        },
                        _ => continue
                    }
                    if parencount == 0_u {
                        // println!("parencount is zero");
                        for k in ::std::iter::range_step_inclusive((j as int)-1, 0, (-1)) {
                            match tokens[(k as uint)].toktype {
                                ::token::PLUS => return parse_plus(getleft(tokens.slice_to((k as uint))), getright(tokens.slice_from((k as uint)+1))),
                                ::token::MINUS => return parse_minus(getleft(tokens.slice_to((k as uint))), getright(tokens.slice_from((k as uint)+1))),
                                _ => continue
                            }
                        }
                        for k in ::std::iter::range_step_inclusive(j as int, 0, (-1)) {
                            match tokens[(k as uint)].toktype {
                                ::token::STAR => return parse_star(getleft(tokens.slice_to((k as uint))), getright(tokens.slice_from((k as uint)+1))),
                                ::token::SLASH => return parse_slash(getleft(tokens.slice_to((k as uint))), getright(tokens.slice_from((k as uint)+1))),
                                _ => continue
                            }
                        }
                        // println!("stripping outer parens");
                        return parse(tokens.slice(1_u, (tokens.len() as uint) - 1_u));
                    }
                }
            },
            _ => continue
        }
    }
    return 5000;
}

pub fn getleft(tokens: &[::token::Token<>]) -> int {
    return parse(tokens);
}

pub fn getright(tokens: &[::token::Token<>]) -> int {
    return parse(tokens);
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
    println!("Subtracting {} with {}", left, right);
    return left - right;
}

pub fn parse_number(numstr: ::std::string::String) -> int {
    match from_str::<int>(numstr.as_slice()) {
        Some(0) => {/*println!("Some(0)");*/ return 0;},
        Some(n) => {/*println!("Some(n)");*/ return n;},
        None => {fail!("Number failed to be parsed, or may not have been a number.");}
    }
}
