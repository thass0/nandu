use std::iter::Peekable;

use logos::Logos;

type Result<T> = std::result::Result<T, ParseError>;

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

pub fn translate(input: impl AsRef<str>) -> Result<String> {
    fn inner(input: &str) -> Result<String> {
        let mut lex = Token::lexer(input).peekable();

        let ast = func(&mut lex).unwrap();

        let _nand_ast = to_nand(ast);

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
            Err(ParseError::UnexpectedToken($lex.next()))
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tree {
    this:     Token,
    branches: Vec<Tree>,
}

impl Tree {
    // Create a tree without any branches.
    fn leaf(token: Token) -> Self {
        Self {
            this:     token,
            branches: vec![],
        }
    }

    // Create a tree with a given set of branches.
    fn new(token: Token, branches: Vec<Tree>) -> Self {
        Self {
            this: token,
            branches,
        }
    }
}

fn to_nand(mut tree: Tree) -> Tree {
    let mut branches = Vec::with_capacity(tree.branches.len());
    for branch in tree.branches.into_iter() {
        branches.push(to_nand(branch));
    }
    tree.branches = branches;

    if tree.this == Token::FuncIdent("And".to_owned()) {
        and_to_nand(tree)
    } else if tree.this == Token::FuncIdent("Or".to_owned()) {
        or_to_nand(tree)
    } else if tree.this == Token::FuncIdent("Nand".to_owned())
        || tree.branches.len() == 0
    {
        tree
    } else {
        panic!("Unexpected Token")
    }
}

fn and_to_nand(mut and_tree: Tree) -> Tree {
    assert_eq!(and_tree.this, Token::FuncIdent("And".to_owned()));
    assert_eq!(and_tree.branches.len(), 2);

    // Create two Nand trees from the And tree.
    and_tree.this = Token::FuncIdent("Nand".to_owned());
    let inner_nand_1 = and_tree.clone();
    let inner_nand_2 = and_tree;

    nand(vec![inner_nand_1, inner_nand_2])
}

fn or_to_nand(mut or_tree: Tree) -> Tree {
    assert_eq!(or_tree.this, Token::FuncIdent("Or".to_owned()));
    assert_eq!(or_tree.branches.len(), 2);

    let arg_2 = or_tree.branches.pop().unwrap();
    let arg_1 = or_tree.branches.pop().unwrap();
    let inner_nand_1 = nand(vec![arg_1.clone(), arg_1]);
    let inner_nand_2 = nand(vec![arg_2.clone(), arg_2]);
    nand(vec![inner_nand_1, inner_nand_2])
}

// Helper to create a new Nand function tree
#[inline]
fn nand(args: Vec<Tree>) -> Tree {
    Tree::new(Token::FuncIdent("Nand".to_owned()), args)
}

// Start symbol.
// Rule: `<F> ::= FuncIdent LParen <ArgList> RParen`
fn func(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Tree> {
    let token = expect!(Some(Token::FuncIdent(_)), lex)?;
    expect!(Some(Token::LParen), lex)?;
    let args = arg_list(lex)?;
    expect!(Some(Token::RParen), lex)?;
    let tree = Tree::new(token, args);
    Ok(tree)
}

// Rule: `<Arg> (Delim <Arg>)*`
// In the AST this function is not represented as a node
// on its own. Instead this function returns all arguments
// as a list of branches.
fn arg_list(
    lex: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Tree>> {
    let mut args = vec![];
    args.push(arg(lex)?);
    while let Some(Token::Delim) = lex.peek() {
        consume(lex)?;
        args.push(arg(lex)?);
    }
    Ok(args)
}

// Rule: `VarIdent | <F>`
fn arg(lex: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Tree> {
    if let Some(Token::VarIdent(_)) = lex.peek() {
        let token = consume(lex)?;
        Ok(Tree::leaf(token))
    } else if let Some(Token::FuncIdent(_)) = lex.peek() {
        func(lex)
    } else {
        Err(ParseError::UnexpectedToken(lex.next()))
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
    fn and_to_nand_works() {
        let and_tree = Tree {
            this:     Token::FuncIdent("And".to_owned()),
            branches: vec![
                Tree::leaf(Token::VarIdent("a".to_owned())),
                Tree::leaf(Token::VarIdent("b".to_owned())),
            ],
        };
        let expected_nand_tree = Tree {
            this:     Token::FuncIdent("Nand".to_owned()),
            branches: vec![
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                        Tree::leaf(Token::VarIdent("b".to_owned())),
                    ],
                },
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                        Tree::leaf(Token::VarIdent("b".to_owned())),
                    ],
                },
            ],
        };
        let result_nand_tree = and_to_nand(and_tree);
        assert_eq!(result_nand_tree, expected_nand_tree);
    }

    #[test]
    fn or_to_nand_works() {
        let or_tree = Tree {
            this:     Token::FuncIdent("Or".to_owned()),
            branches: vec![
                Tree::leaf(Token::VarIdent("a".to_owned())),
                Tree::leaf(Token::VarIdent("b".to_owned())),
            ],
        };
        let expected_nand_tree = Tree {
            this:     Token::FuncIdent("Nand".to_owned()),
            branches: vec![
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                    ],
                },
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("b".to_owned())),
                        Tree::leaf(Token::VarIdent("b".to_owned())),
                    ],
                },
            ],
        };
        let result_nand_tree = or_to_nand(or_tree);
        assert_eq!(result_nand_tree, expected_nand_tree);
    }

    #[test]
    fn generic_tree_to_nand_works() {
        let tree = Tree {
            this:     Token::FuncIdent("And".to_owned()),
            branches: vec![Tree::leaf(Token::VarIdent("a".to_owned())), Tree {
                this:     Token::FuncIdent("Or".to_owned()),
                branches: vec![
                    Tree::leaf(Token::VarIdent("b".to_owned())),
                    Tree::leaf(Token::VarIdent("c".to_owned())),
                ],
            }],
        };
        let expected_nand_tree = Tree {
            this:     Token::FuncIdent("Nand".to_owned()),
            branches: vec![
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                        Tree {
                            this:     Token::FuncIdent("Nand".to_owned()),
                            branches: vec![
                                Tree {
                                    this:     Token::FuncIdent(
                                        "Nand".to_owned(),
                                    ),
                                    branches: vec![
                                        Tree::leaf(Token::VarIdent(
                                            "b".to_owned(),
                                        )),
                                        Tree::leaf(Token::VarIdent(
                                            "b".to_owned(),
                                        )),
                                    ],
                                },
                                Tree {
                                    this:     Token::FuncIdent(
                                        "Nand".to_owned(),
                                    ),
                                    branches: vec![
                                        Tree::leaf(Token::VarIdent(
                                            "c".to_owned(),
                                        )),
                                        Tree::leaf(Token::VarIdent(
                                            "c".to_owned(),
                                        )),
                                    ],
                                },
                            ],
                        },
                    ],
                },
                Tree {
                    this:     Token::FuncIdent("Nand".to_owned()),
                    branches: vec![
                        Tree::leaf(Token::VarIdent("a".to_owned())),
                        Tree {
                            this:     Token::FuncIdent("Nand".to_owned()),
                            branches: vec![
                                Tree {
                                    this:     Token::FuncIdent(
                                        "Nand".to_owned(),
                                    ),
                                    branches: vec![
                                        Tree::leaf(Token::VarIdent(
                                            "b".to_owned(),
                                        )),
                                        Tree::leaf(Token::VarIdent(
                                            "b".to_owned(),
                                        )),
                                    ],
                                },
                                Tree {
                                    this:     Token::FuncIdent(
                                        "Nand".to_owned(),
                                    ),
                                    branches: vec![
                                        Tree::leaf(Token::VarIdent(
                                            "c".to_owned(),
                                        )),
                                        Tree::leaf(Token::VarIdent(
                                            "c".to_owned(),
                                        )),
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        let result_nand_tree = to_nand(tree);
        assert_eq!(result_nand_tree, expected_nand_tree);
    }

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

    #[test]
    fn parse_simple_ast() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::VarIdent("a".to_owned()),
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        let expected_tree = Tree {
            this:     Token::FuncIdent("A".to_owned()),
            branches: vec![
                Tree::leaf(Token::VarIdent("a".to_owned())),
                Tree::leaf(Token::VarIdent("b".to_owned())),
            ],
        };
        let result_tree = func(&mut token_stream).unwrap();
        assert_eq!(result_tree, expected_tree);
    }

    #[test]
    fn parse_nested_ast() {
        let mut token_stream = [
            Token::FuncIdent("A".to_owned()),
            Token::LParen,
            Token::FuncIdent("B".to_owned()),
            Token::LParen,
            Token::FuncIdent("C".to_owned()),
            Token::LParen,
            Token::VarIdent("c".to_string()),
            Token::RParen,
            Token::RParen,
            Token::Delim,
            Token::VarIdent("a".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        let expected_tree = Tree {
            this:     Token::FuncIdent("A".to_owned()),
            branches: vec![
                Tree {
                    this:     Token::FuncIdent("B".to_owned()),
                    branches: vec![Tree {
                        this:     Token::FuncIdent("C".to_owned()),
                        branches: vec![Tree::leaf(Token::VarIdent(
                            "c".to_owned(),
                        ))],
                    }],
                },
                Tree::leaf(Token::VarIdent("a".to_owned())),
            ],
        };
        let result_tree = func(&mut token_stream).unwrap();
        assert_eq!(result_tree, expected_tree);
    }

    #[test]
    fn tree_leaf_create_tree_without_branches() {
        // The specific token doesn't matter here.
        let test_token = Token::VarIdent("blah".to_owned());
        let automatic_leaf = Tree::leaf(test_token.clone());
        let manual_leaf = Tree {
            this:     test_token.clone(),
            branches: vec![],
        };
        assert_eq!(automatic_leaf, manual_leaf);
    }
}
