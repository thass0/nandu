use crate::lex::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    this:     Token,
    branches: Vec<Tree>,
}

impl Tree {
    // Create a tree without any branches.
    pub fn leaf(token: Token) -> Self {
        Self {
            this:     token,
            branches: vec![],
        }
    }

    // Create a tree with a given set of branches.
    pub fn new(token: Token, branches: Vec<Tree>) -> Self {
        Self {
            this: token,
            branches,
        }
    }
}

pub fn to_string(tree: Tree) -> String {
    match tree.this {
        Token::FuncIdent(id) => {
            let mut buf = format!("{id}(");
            let mut branches_iter = tree.branches.into_iter();
            buf.push_str(&to_string(branches_iter.next().expect(
                "valid parsed functions must contain at least one argument",
            )));
            for branch in branches_iter {
                buf.push_str(&format!(", {}", to_string(branch)));
            }
            buf.push_str(")");
            buf
        },
        Token::VarIdent(id) => id,
        _ => panic!("unexpected token in tree branch"),
    }
}

pub fn to_nand(mut tree: Tree) -> Tree {
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
        panic!("unexpected token in tree branch")
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
        let result_tree = crate::parse::start(&mut token_stream).unwrap();
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
        let result_tree = crate::parse::start(&mut token_stream).unwrap();
        assert_eq!(result_tree, expected_tree);
    }
}
