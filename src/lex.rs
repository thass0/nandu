use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Delim,
    // A function's identifier must be at least two characters
    // long. The first character must be a capital letter.
    // The case of all the subsequent letters is irrelevant.
    #[regex("[A-Z][A-Za-z]+", |lex| lex.slice().to_string())]
    FuncIdent(String),
    // A variable's identifier must be at least one character
    // long. The first character must be a lowercase letter.
    // All subsequent characters can be either lowercase letters
    // or underscores.
    #[regex("[a-z][a-z_]*", |lex| lex.slice().to_string())]
    VarIdent(String),
    #[error]
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    LexError,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LParen => write!(f, "'('"),
            Self::RParen => write!(f, "')'"),
            Self::Delim => write!(f, "','"),
            Self::FuncIdent(id) => write!(f, "function '{id}'"),
            Self::VarIdent(id) => write!(f, "variable '{id}'"),
            Self::LexError => write!(f, "lexical error"),
        }
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        match self {
            Token::FuncIdent(id) | Token::VarIdent(id) => id.as_str(),
            _ => "",
        }
    }
}
