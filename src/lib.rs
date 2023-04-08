use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
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

pub fn translate(
    input: impl AsRef<str>,
) -> Result<String, Box<dyn std::error::Error>> {
    fn inner(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut lex = Token::lexer(input);
        while let Some(token) = lex.next() {
            dbg!(&token);
        }

        todo!("Parse and translate the input")
    }

    inner(input.as_ref())
}
