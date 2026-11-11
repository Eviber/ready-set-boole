mod node;

/// Parsed boolean expression, which can be evaluated.
#[derive(Debug, PartialEq, Eq)]
pub struct BooleanExpression {
    root: node::Node,
    variables: Vec<char>,
}

impl BooleanExpression {
    /// Evaluates the expression.
    ///
    /// If there are variables, all are set to false
    pub fn eval(&self) -> bool {
        self.root.eval()
    }

    /// Evaluates the expression with each variable set according the the bitfield `vars`.
    ///
    /// Each existing variable is set with the last unused bit, in reverse alphabetical order.
    ///
    /// For the expression `CX&G^`, `vars` define the variables as such:
    /// `0000_0000_0000_0000_0000_0000_0000_0CGX`
    pub fn eval_vars(&self, vars: u32) -> bool {
        self.root.eval_vars(vars)
    }

    /// Returns a slice of all variables present in the expression, in alphabetical order.
    pub fn variables(&self) -> &[char] {
        &self.variables
    }

    /// Returns the number of different variables in the expression.
    ///
    /// Note: a variable with multiple instances is still one variable.
    /// This means that `AB^A|` has 2 variables, not 3.
    pub fn variables_count(&self) -> usize {
        self.variables.len()
    }
}

impl std::str::FromStr for BooleanExpression {
    type Err = node::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = s.parse()?;
        let variables: Vec<_> = ('A'..='Z').filter(|&c| s.contains(c)).collect();
        Ok(BooleanExpression { root, variables })
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
