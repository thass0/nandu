use crate::parse::Id;

// Single node in a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Func { id: Id, args: Vec<Node> },
    Var { id: String },
}

// implement all the tree transformations for Node
impl Node {
    pub fn to_nand(&mut self) {
        match self {
            Node::Func { id, args } => {
                for arg in args.iter_mut() {
                    arg.to_nand();
                }

                match id {
                    Id::And => {
                        debug_assert_eq!(args.len(), 2);
                        let nested_1 = Node::Func {
                            id:   Id::Nand,
                            args: args.clone(),
                        };
                        let nested_2 = Node::Func {
                            id:   Id::Nand,
                            args: args.clone(),
                        };
                        *self = Node::Func {
                            id:   Id::Nand,
                            args: vec![nested_1, nested_2],
                        };
                    },
                    Id::Or => {
                        debug_assert_eq!(args.len(), 2);
                        let nested_1 = Node::Func {
                            id:   Id::Nand,
                            args: vec![args[0].clone(), args[0].clone()],
                        };
                        let nested_2 = Node::Func {
                            id:   Id::Nand,
                            args: vec![args[1].clone(), args[1].clone()],
                        };
                        *self = Node::Func {
                            id:   Id::Nand,
                            args: vec![nested_1, nested_2],
                        };
                    },
                    Id::Nand => {},
                }
            },
            Node::Var { .. } => {},
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Func { id, args } => {
                let mut args_str = String::new();
                for arg in args.iter().take(1) {
                    args_str.push_str(&arg.to_string());
                }
                for arg in args.iter().skip(1) {
                    args_str.push_str(&format!(", {arg}"));
                }

                write!(f, "{id}({args_str})")
            },
            Node::Var { id } => write!(f, "{id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::Token;

    #[test]
    fn and_to_nand_works() {
        let mut and_tree = Node::Func {
            id:   Id::And,
            args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                id: "b".to_owned(),
            }],
        };
        let expected_nand_tree = Node::Func {
            id:   Id::Nand,
            args: vec![
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                        id: "b".to_owned(),
                    }],
                },
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                        id: "b".to_owned(),
                    }],
                },
            ],
        };
        and_tree.to_nand();
        assert_eq!(and_tree, expected_nand_tree);
    }

    #[test]
    fn or_to_nand_works() {
        let mut or_tree = Node::Func {
            id:   Id::Or,
            args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                id: "b".to_owned(),
            }],
        };
        let expected_nand_tree = Node::Func {
            id:   Id::Nand,
            args: vec![
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                        id: "a".to_owned(),
                    }],
                },
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "b".to_owned() }, Node::Var {
                        id: "b".to_owned(),
                    }],
                },
            ],
        };
        or_tree.to_nand();
        assert_eq!(or_tree, expected_nand_tree);
    }

    #[test]
    fn generic_tree_to_nand_works() {
        let mut tree = Node::Func {
            id:   Id::And,
            args: vec![Node::Var { id: "a".to_owned() }, Node::Func {
                id:   Id::Or,
                args: vec![Node::Var { id: "b".to_owned() }, Node::Var {
                    id: "c".to_owned(),
                }],
            }],
        };
        let expected_nand_tree = Node::Func {
            id:   Id::Nand,
            args: vec![
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "a".to_owned() }, Node::Func {
                        id:   Id::Nand,
                        args: vec![
                            Node::Func {
                                id:   Id::Nand,
                                args: vec![
                                    Node::Var { id: "b".to_owned() },
                                    Node::Var { id: "b".to_owned() },
                                ],
                            },
                            Node::Func {
                                id:   Id::Nand,
                                args: vec![
                                    Node::Var { id: "c".to_owned() },
                                    Node::Var { id: "c".to_owned() },
                                ],
                            },
                        ],
                    }],
                },
                Node::Func {
                    id:   Id::Nand,
                    args: vec![Node::Var { id: "a".to_owned() }, Node::Func {
                        id:   Id::Nand,
                        args: vec![
                            Node::Func {
                                id:   Id::Nand,
                                args: vec![
                                    Node::Var { id: "b".to_owned() },
                                    Node::Var { id: "b".to_owned() },
                                ],
                            },
                            Node::Func {
                                id:   Id::Nand,
                                args: vec![
                                    Node::Var { id: "c".to_owned() },
                                    Node::Var { id: "c".to_owned() },
                                ],
                            },
                        ],
                    }],
                },
            ],
        };
        tree.to_nand();
        assert_eq!(tree, expected_nand_tree);
    }

    #[test]
    fn parse_simple_ast() {
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
        let expected_tree = Node::Func {
            id:   Id::And,
            args: vec![Node::Var { id: "a".to_owned() }, Node::Var {
                id: "b".to_owned(),
            }],
        };
        let result_tree = crate::parse::start(&mut token_stream).unwrap();
        assert_eq!(result_tree, expected_tree);
    }

    #[test]
    fn parse_nested_ast() {
        let mut token_stream = [
            Token::FuncIdent("And".to_owned()),
            Token::LParen,
            Token::FuncIdent("Or".to_owned()),
            Token::LParen,
            Token::FuncIdent("Nand".to_owned()),
            Token::LParen,
            Token::VarIdent("c".to_owned()),
            Token::Delim,
            Token::VarIdent("d".to_owned()),
            Token::RParen,
            Token::Delim,
            Token::VarIdent("b".to_owned()),
            Token::RParen,
            Token::Delim,
            Token::VarIdent("a".to_owned()),
            Token::RParen,
        ]
        .into_iter()
        .peekable();
        let expected_tree = Node::Func {
            id:   Id::And,
            args: vec![
                Node::Func {
                    id:   Id::Or,
                    args: vec![
                        Node::Func {
                            id:   Id::Nand,
                            args: vec![
                                Node::Var { id: "c".to_owned() },
                                Node::Var { id: "d".to_owned() },
                            ],
                        },
                        Node::Var { id: "b".to_owned() },
                    ],
                },
                Node::Var { id: "a".to_owned() },
            ],
        };
        let result_tree = crate::parse::start(&mut token_stream).unwrap();
        assert_eq!(result_tree, expected_tree);
    }
}
