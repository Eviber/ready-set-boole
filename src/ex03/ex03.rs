// an AST to parse logical expressions in rpn

#[derive(Debug)]
enum BinOp {
    And,
    Or,
    Xor,
    Impl,
    Xnor,
}

#[derive(Debug)]
enum UnaryOp {
    Not,
}

#[derive(Debug)]
enum Constant {
    bool,
    Vec<i32>,
}

#[derive(Debug)]
enum Value {
    Const(Constant),
    Var(char),
}

#[derive(Debug)]
enum Node {
    Binary { op: BinOp, left: Box<Node>, right: Box<Node> },
    Unary { op: UnaryOp, operand: Box<Node> },
    Val(Value),
}

use Node::*;
use BinOp::*;
use UnaryOp::*;
use Value::*;
use Constant::*;

fn main() {
    let tree = Binary {
        op: And,
        left: Box::new(Val(Const(True))),
        right: Box::new(Binary {
            op: Or,
            left: Box::new(Val(Const(False))),
            right: Box::new(Unary {
                op: Not,
                operand: Box::new(Val(Var('x'))),
            }),
        }),
    };
    println!("{:?}", tree);
}
