use std::iter::Peekable;

use logos::Logos;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Logos, Debug, Clone, PartialEq)]
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

pub fn translate(input: impl AsRef<str>) -> Result<String> {
    fn inner(input: &str) -> Result<String> {
        let mut lex = Token::lexer(input).peekable();

        func(&mut lex).unwrap();

        Ok(input.to_owned())
    }

    inner(input.as_ref())
}

// Parser subroutine to either consume the
// expected token or throw an error.
macro_rules! expect {
    ($expected:pat, $lex:expr) => {
        if let $expected = $lex.peek() {
            consume($lex)
        } else {
            Err(ParseError::UnexpectedToken(
                $lex.peek().map(|token| token.clone()),
            ))
        }
    };
}

// Start symbol.
// Rule: `<F> ::= FuncIdent LParen <ArgList> RParen`
fn func(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<()> {
    expect!(Some(Token::FuncIdent(_)), lex)?;
    expect!(Some(Token::LParen), lex)?;
    arg_list(lex)?;
    expect!(Some(Token::RParen), lex)?;
    Ok(())
}

// Rule: `<Arg> (Delim <Arg>)*`
fn arg_list(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<()> {
    arg(lex)?;
    while let Some(Token::Delim) = lex.peek() {
        consume(lex)?;
        arg(lex)?;
    }
    Ok(())
}

// Rule: `VarIdent | <F>`
fn arg(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<()> {
    if let Some(Token::VarIdent(_)) = lex.peek() {
        consume(lex)?;
        Ok(())
    } else if let Some(Token::FuncIdent(_)) = lex.peek() {
        func(lex)
    } else {
        Err(ParseError::UnexpectedToken(
            lex.peek().map(|token| token.clone()),
        ))
    }
}

// Consume the current lookahead and advance the token
// stream. Returns the consumed token or raises and error,
// if the token stream has ended.
#[inline]
fn consume(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Token> {
    lex.next().ok_or(ParseError::UnexpectedEnd)
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Option<Token>),
    UnexpectedEnd,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token {token:?}")
            },
            Self::UnexpectedEnd => {
                write!(f, "unexpected end")
            },
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_simple_function() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        assert!(func(&mut token_stream).is_ok());
    }

    #[test]
    fn parse_accepts_multi_arg_function() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::Delim,
            Token::VarIdent("c".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        assert!(func(&mut token_stream).is_ok());
    }

    #[test]
    fn parse_accepts_simple_nested_function() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::FuncIdent("B".to_owned()),
            Token::LParen,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        assert!(func(&mut token_stream).is_ok());
    }
    #[test]
    fn parse_accepts_nested_function_next_to_args() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::FuncIdent("B".to_owned()),
            Token::LParen,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
            Token::Delim,
            Token::VarIdent("a".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        assert!(func(&mut token_stream).is_ok());
    }
    #[test]
    fn parse_accepts_multiple_nestings() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::FuncIdent("B".to_owned()),
            Token::LParen,
            Token::FuncIdent("C".to_owned()),
            Token::LParen,
            Token::FuncIdent("D".to_owned()),
            Token::LParen,
            Token::VarIdent("d".to_string()),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        assert!(func(&mut token_stream).is_ok());
    }
}
