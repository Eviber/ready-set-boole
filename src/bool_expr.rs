mod node;

#[derive(Debug, PartialEq, Eq)]
pub struct BooleanExpression(node::Node);

impl BooleanExpression {
    pub fn eval(&self) -> bool {
        self.0.eval()
    }
}

impl std::str::FromStr for BooleanExpression {
    type Err = node::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BooleanExpression(s.parse()?))
    }
}

#[cfg(test)]
mod test {
    use super::node::BinOpToken::*;
    use super::node::ParseError::*;
    use super::node::Token::*;
    use super::BooleanExpression;

    fn test_err(s: &str, err: super::node::ParseError) {
        assert_eq!(s.parse::<BooleanExpression>(), Err(err));
    }

    #[test]
    fn parsing_errors() {
        test_err("", EmptyExpression);

        test_err("1&", MissingOperand(BinOp(And)));
        test_err("1|", MissingOperand(BinOp(Or)));
        test_err("1>", MissingOperand(BinOp(Imply)));
        test_err("1=", MissingOperand(BinOp(Eq)));
        test_err("1^", MissingOperand(BinOp(Xor)));
        test_err("!", MissingOperand(Not));

        test_err("00&1", UnbalancedExpression(BinOp(And), Literal(true)));
        test_err("101|", UnbalancedExpression(Literal(true), BinOp(Or)));
        test_err("10=0", UnbalancedExpression(BinOp(Eq), Literal(false)));
        test_err("011^", UnbalancedExpression(Literal(false), BinOp(Xor)));
        test_err("00>10&", UnbalancedExpression(BinOp(Imply), BinOp(And)));

        test_err("1x|", UnexpectedCharacterError('x'));
        test_err("1y&", UnexpectedCharacterError('y'));
        test_err("1$>", UnexpectedCharacterError('$'));
        test_err("1;=", UnexpectedCharacterError(';'));
        test_err("1 ^", UnexpectedCharacterError(' '));
        test_err("1+!", UnexpectedCharacterError('+'));
    }
}
