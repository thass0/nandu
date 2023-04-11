mod lex;
mod parse;
mod tree;

use logos::Logos;

use crate::lex::Token;
use crate::parse::{start, ParseError};

type Result<T> = std::result::Result<T, ParseError>;

pub fn translate(input: impl AsRef<str>) -> Result<String> {
    fn inner(input: &str) -> Result<String> {
        let mut lex = Token::lexer(input).peekable();

        let mut ast = start(&mut lex)?;
        ast.to_nand();
        let nand_string = ast.to_string();

        Ok(nand_string)
    }

    inner(input.as_ref())
}
