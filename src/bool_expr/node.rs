mod error;
mod token;

pub use error::ParseError;
pub use token::{BinOpToken, Token};

/// Node of an AST representation of a boolean expression
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Node {
    BinOp {
        op: BinOpToken,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Not(Box<Node>),
    Literal(bool),
    Variable {
        name: char,
        idx: u32,
    },
}

impl Node {
    fn literal(b: bool) -> Node {
        Node::Literal(b)
    }

    fn not(operand: Node) -> Node {
        Node::Not(Box::new(operand))
    }

    fn binop(op: BinOpToken, lhs: Node, rhs: Node) -> Node {
        Node::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub(super) fn eval_vars(&self, vars: u32) -> bool {
        match self {
            Node::BinOp { op, lhs, rhs } => {
                let lhs = lhs.eval_vars(vars);
                let rhs = rhs.eval_vars(vars);
                match op {
                    BinOpToken::And => lhs && rhs,
                    BinOpToken::Or => lhs || rhs,
                    BinOpToken::Xor => lhs ^ rhs,
                    BinOpToken::Imply => !lhs || rhs,
                    BinOpToken::Eq => lhs == rhs,
                }
            }
            Node::Not(operand) => !operand.eval_vars(vars),
            Node::Literal(b) => *b,
            Node::Variable { name: _, idx } => vars >> idx & 1 == 1,
        }
    }

    pub(super) fn eval(&self) -> bool {
        self.eval_vars(0)
    }
}

impl From<Node> for Token {
    fn from(value: Node) -> Self {
        match value {
            Node::BinOp { op, .. } => Token::BinOp(op),
            Node::Not(_) => Token::Not,
            Node::Literal(c) => Token::Literal(c),
            Node::Variable { name, .. } => Token::Variable(name),
        }
    }
}

impl std::str::FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(s.len());
        let mut tokens = Vec::new();
        let mut vars = [false; 26];

        for token in token::tokenize(s) {
            let token = token?;
            if let Token::Variable(c) = token {
                assert!(c.is_ascii_uppercase());
                vars[(c as u8 - b'A') as usize] = true;
            }
            tokens.push(token);
        }

        let mut vars_idx = [None; 26];
        let mut idx = 0;
        for c in ('A'..='Z').rev() {
            let i = (c as u8 - b'A') as usize;
            if vars[i] {
                vars_idx[i] = Some(idx);
                idx += 1;
            }
        }

        for token in tokens {
            let node = match token {
                Token::Variable(c) => Node::Variable {
                    name: c,
                    idx: vars_idx[(c as u8 - b'A') as usize]
                        .expect("All variables should be indexed"),
                },
                Token::Literal(b) => Node::literal(b),
                Token::Not => {
                    let operand = stack.pop().ok_or(ParseError::MissingOperand(token))?;
                    Node::not(operand)
                }
                Token::BinOp(binop_token) => {
                    let rhs = stack.pop().ok_or(ParseError::MissingOperand(token))?;
                    let lhs = stack.pop().ok_or(ParseError::MissingOperand(token))?;
                    Node::binop(binop_token, lhs, rhs)
                }
            };
            stack.push(node);
        }
        if stack.len() > 1 {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            return Err(ParseError::UnbalancedExpression(lhs.into(), rhs.into()));
        }
        stack.pop().ok_or(ParseError::EmptyExpression)
    }
}
