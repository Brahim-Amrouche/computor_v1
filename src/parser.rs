use core::fmt;

use crate::tokenizer::{PolynomialsToken, Tokenizer};

pub struct Parser {
    tokens: Tokenizer,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<usize>,
    pub lhs: Option<usize>,
    pub rhs: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ParseTree<T> {
    pub tree: Vec<Node<T>>,
    pub root: Option<usize>,
}

impl<T> ParseTree<T> {
    pub fn new() -> Self {
        ParseTree {
            tree: Vec::new(),
            root: None,
        }
    }

    pub fn add_node(&mut self, node: Node<T>) -> usize {
        let idx = self.tree.len();
        self.tree.push(node);
        idx
    }

    fn print_node(
        &self,
        f: &mut fmt::Formatter<'_>,
        node_idx: usize,
        prefix: String,
        is_last: bool,
        is_root: bool,
    ) -> fmt::Result
    where
        T: fmt::Display,
    {
        let node = &self.tree[node_idx];

        if is_root {
            writeln!(f, "{}", node.value)?;
        } else {
            let connector = if is_last { "└── " } else { "├── " };
            writeln!(f, "{}{}{}", prefix, connector, node.value)?;
        }

        let mut new_prefix = prefix;
        if !is_root {
            new_prefix.push_str(if is_last { "    " } else { "│   " });
        }

        let mut children = Vec::new();
        if let Some(l) = node.lhs {
            children.push(l);
        }
        if let Some(r) = node.rhs {
            children.push(r);
        }

        for (i, &child_idx) in children.iter().enumerate() {
            let child_is_last = i == children.len() - 1;
            self.print_node(f, child_idx, new_prefix.clone(), child_is_last, false)?;
        }

        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for ParseTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root_idx) = self.root {
            self.print_node(f, root_idx, String::new(), true, true)
        } else {
            write!(f, "Empty ParseTree")
        }
    }
}

#[derive(Debug)]
pub enum ParsingError {
    ExpressionError(String),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::ExpressionError(msg) => write!(f, "Wrongly Formated Expression: {}", msg),
        }
    }
}

impl std::error::Error for ParsingError {}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Parser { tokens: tokenizer }
    }

    pub fn parse(&mut self) -> Result<ParseTree<PolynomialsToken>, ParsingError> {
        let mut tree = ParseTree::new();
        let root = self.parse_expr(&mut tree, 0)?;
        tree.root = Some(root);

        if self.tokens.get_next_token().is_some() {
            return Err(ParsingError::ExpressionError(
                "Extra tokens after expression".to_string(),
            ));
        }

        Ok(tree)
    }

    fn get_binding_power(op: &PolynomialsToken) -> Option<(u8, u8)> {
        match op {
            PolynomialsToken::Equals => Some((1, 2)),
            PolynomialsToken::Addition | PolynomialsToken::Substraction => Some((3, 4)),
            PolynomialsToken::Multiplication => Some((5, 6)),
            PolynomialsToken::Exponential => Some((8, 7)),
            _ => None,
        }
    }

    fn parse_expr(
        &mut self,
        tree: &mut ParseTree<PolynomialsToken>,
        min_bp: u8,
    ) -> Result<usize, ParsingError> {
        let lhs_token = self
            .tokens
            .get_next_token()
            .ok_or_else(|| ParsingError::ExpressionError("Unexpected EOF".to_string()))?;

        let mut lhs_idx = match lhs_token {
            PolynomialsToken::Addition | PolynomialsToken::Substraction => {
                let r_bp = 9;
                let rhs = self.parse_expr(tree, r_bp)?;
                let node = Node {
                    value: lhs_token,
                    parent: None,
                    lhs: None,
                    rhs: Some(rhs),
                };
                let idx = tree.add_node(node);
                tree.tree[rhs].parent = Some(idx);
                idx
            }
            PolynomialsToken::Expression(_) => tree.add_node(Node {
                value: lhs_token,
                parent: None,
                lhs: None,
                rhs: None,
            }),
            _ => {
                return Err(ParsingError::ExpressionError(format!(
                    "Unexpected token: {:?}",
                    lhs_token
                )));
            }
        };

        while let Some(op) = self.tokens.peek() {
            if let Some((l_bp, r_bp)) = Self::get_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                let op_token = self.tokens.get_next_token().unwrap();
                let rhs_idx = self.parse_expr(tree, r_bp)?;

                let node = Node {
                    value: op_token,
                    parent: None,
                    lhs: Some(lhs_idx),
                    rhs: Some(rhs_idx),
                };
                let new_idx = tree.add_node(node);
                tree.tree[lhs_idx].parent = Some(new_idx);
                tree.tree[rhs_idx].parent = Some(new_idx);

                lhs_idx = new_idx;
            } else {
                break;
            }
        }

        Ok(lhs_idx)
    }
}
