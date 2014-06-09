pub enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    STAR,
    SLASH
}

pub struct Token {
    pub value: ::std::string::String,
    pub toktype: TokenType
}

impl ::std::fmt::Show for Token {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
