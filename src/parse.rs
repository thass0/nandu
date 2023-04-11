use std::iter::Peekable;

use crate::lex::Token;
use crate::tree::Node;
use crate::Result;

pub const AND_ID: &str = "And";
pub const OR_ID: &str = "Or";
pub const NAND_ID: &str = "Nand";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Id {
    And,
    Or,
    Nand,
}

impl Id {
    // NOTE: Custom function can be implemented later by
    // allowing user-defined function identifiers here.
    pub fn parse(id: &str, num_args: usize) -> Option<Self> {
        match id {
            AND_ID if num_args == 2 => Some(Self::And),
            OR_ID if num_args == 2 => Some(Self::Or),
            NAND_ID if num_args == 2 => Some(Self::Nand),
            _ => None,
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "{AND_ID}"),
            Self::Or => write!(f, "{OR_ID}"),
            Self::Nand => write!(f, "{NAND_ID}"),
        }
    }
}

// Parser subroutine to either consume the
// expected token or throw an error.
macro_rules! expect {
    ($expected:pat, $lex:expr) => {
        if let $expected = $lex.peek() {
            consume($lex)
        } else {
            Err(ParseError::UnexpectedToken($lex.next()))
        }
    };
}

// Start symbol.
// Rule: `<S> ::= <F> end`.
// `end` means that the input is over, so in
// this case that `lex.peek` is `None`.
pub fn start(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Node> {
    let tree = func(lex)?;
    if lex.peek().is_none() {
        consume(lex).err();
    } else {
        return Err(ParseError::UnexpectedToken(lex.next()));
    }
    Ok(tree)
}

// Rule: `<F> ::= FuncIdent LParen <ArgList> RParen`
fn func(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Node> {
    let token = expect!(Some(Token::FuncIdent(_)), lex)?;
    expect!(Some(Token::LParen), lex)?;
    let args = arg_list(lex)?;
    expect!(Some(Token::RParen), lex)?;
    let token_id = token.as_ref();
    let id = Id::parse(token_id, args.len())
        .ok_or(ParseError::InvalidFunctionId(token_id.to_owned()))?;
    Ok(Node::Func { id, args })
}

// Rule: `<Arg> (Delim <Arg>)*`
// In the AST this function is not represented as a node
// on its own. Instead this function returns all arguments
// as a list of branches.
fn arg_list(
    lex: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Node>> {
    let mut args = vec![];
    args.push(arg(lex)?);
    while let Some(Token::Delim) = lex.peek() {
        consume(lex)?;
        args.push(arg(lex)?);
    }
    Ok(args)
}

// Rule: `VarIdent | <F>`
fn arg(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Node> {
    if let Some(Token::VarIdent(_)) = lex.peek() {
        let token = consume(lex)?;
        let node = Node::Var {
            id: token.as_ref().to_owned(),
        };
        Ok(node)
    } else if let Some(Token::FuncIdent(_)) = lex.peek() {
        func(lex)
    } else {
        Err(ParseError::UnexpectedToken(lex.next()))
    }
}

// Consume the current lookahead and advance the token
// stream. Returns the consumed token or returns `None`
// if the token stream has ended.
#[inline]
fn consume(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Token> {
    lex.next().ok_or(ParseError::UnexpectedEnd)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken(Option<Token>),
    InvalidFunctionId(String),
    UnexpectedEnd,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => match token {
                Some(t) => write!(f, "unexpected token {t}"),
                None => write!(f, "unexpected missing token"),
            },
            Self::InvalidFunctionId(id) => {
                write!(f, "unknown function id '{id}'")
            },
            Self::UnexpectedEnd => {
                write!(f, "unexpected end of input")
            },
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_and() {
        let mut token_stream = [
            Token::FuncIdent("And".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        func(&mut token_stream).unwrap();
    }

    #[test]
    fn parse_accepts_or() {
        let mut token_stream = [
            Token::FuncIdent("Or".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        func(&mut token_stream).unwrap();
    }

    #[test]
    fn parse_accepts_nand() {
        let mut token_stream = [
            Token::FuncIdent("Nand".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        func(&mut token_stream).unwrap();
    }

    #[test]
    fn parse_accepts_nested_functions() {
        let mut token_stream = [
            Token::FuncIdent("And".to_owned()),
            Token::LParen,
            Token::FuncIdent("Or".to_owned()),
            Token::LParen,
            Token::VarIdent("b".to_owned()),
            Token::Delim,
            Token::VarIdent("c".to_owned()),
            Token::RParen,
            Token::Delim,
            Token::VarIdent("a".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        func(&mut token_stream).unwrap();
    }
}
