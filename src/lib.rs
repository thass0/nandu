#![feature(test)]

extern crate test;

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

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_lots_of_nested_ands(b: &mut Bencher) {
        let ands = "And(a, b)\n";
        let tokens: Vec<Token> = Token::lexer(ands).collect();
        b.iter(|| {
            let mut ast =
                start(&mut tokens.iter().cloned().peekable()).unwrap();
            ast.to_nand();
            ast.to_string();
        });
    }
}
