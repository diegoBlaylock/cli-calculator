#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus,
}

#[derive(Debug, PartialEq)]
pub enum MathExpression {
    BinOp {
        op: BinOp,
        lhs: Box<MathExpression>,
        rhs: Box<MathExpression>,
    },
    Variable {
        name: String,
    },
    Function {
        name: String,
        arguments: Vec<MathExpression>,
    },
    Number(f64),
}
