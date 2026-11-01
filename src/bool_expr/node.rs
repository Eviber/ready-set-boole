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

    pub(super) fn eval(&self) -> bool {
        match self {
            Node::BinOp { op, lhs, rhs } => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                match op {
                    BinOpToken::And => lhs && rhs,
                    BinOpToken::Or => lhs || rhs,
                    BinOpToken::Xor => lhs ^ rhs,
                    BinOpToken::Imply => !lhs || rhs,
                    BinOpToken::Eq => lhs == rhs,
                }
            }
            Node::Not(operand) => !operand.eval(),
            Node::Literal(b) => *b,
        }
    }
}

impl std::str::FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::with_capacity(s.len());

        for token in token::tokenize(s) {
            let token = token?;
            let node = match token {
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
            return Err(ParseError::UnbalancedExpression);
        }
        stack.pop().ok_or(ParseError::EmptyExpression)
    }
}
