pub enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAREN,
    RPAREN
}

pub struct Token<'a> {
    pub value: &'a str,
    pub toktype: TokenType
}

impl<'a> ::std::fmt::Show for Token<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> Token<'a> {
    pub fn is_terminal(&self) -> bool {
        return match self.toktype {
            NUMBER => true,
            LPAREN => true,
            RPAREN => true,
            _ => false
        }
    }
}
