mod lex;
mod parse;
mod tree;

use logos::Logos;

use crate::lex::Token;
use crate::parse::{start, ParseError};
use crate::tree::{to_nand, to_string};

type Result<T> = std::result::Result<T, ParseError>;

pub fn translate(input: impl AsRef<str>) -> Result<String> {
    fn inner(input: &str) -> Result<String> {
        let mut lex = Token::lexer(input).peekable();

        let ast = start(&mut lex)?;
        let nand_ast = to_nand(ast);
        let nand_string = to_string(nand_ast);

        Ok(nand_string)
    }

    inner(input.as_ref())
}
