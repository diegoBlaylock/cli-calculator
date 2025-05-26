use std::collections::HashMap;

use crate::interp::{ast::MathExpression, interpreter::interpret, parse::expression_p};

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::{interpreter::error::InterpreterError, *};

#[test]
fn test_parse() -> Result<(), nom::Err<&'static str>> {
    assert_eq!(expression_p("1.21")?, MathExpression::Number(1.21));
    assert_eq!(expression_p("1")?, MathExpression::Number(1.0));
    assert_eq!(
        expression_p("x")?,
        MathExpression::Variable {
            name: "x".to_string()
        }
    );
    assert_eq!(
        expression_p("(((x)))")?,
        MathExpression::Variable {
            name: "x".to_string()
        }
    );
    assert_eq!(
        expression_p("rand()")?,
        MathExpression::Function {
            name: "rand".to_string(),
            arguments: Vec::new()
        }
    );
    assert_eq!(
        expression_p("1+2")?,
        MathExpression::BinOp {
            op: ast::BinOp::Add,
            lhs: Box::new(MathExpression::Number(1.0)),
            rhs: Box::new(MathExpression::Number(2.0))
        }
    );
    assert_eq!(
        expression_p(" 1 + 2 - 3 + 4")?,
        MathExpression::BinOp {
            op: ast::BinOp::Add,
            lhs: Box::new(MathExpression::BinOp {
                op: ast::BinOp::Subtract,
                lhs: Box::new(MathExpression::BinOp {
                    op: ast::BinOp::Add,
                    lhs: Box::new(MathExpression::Number(1.0)),
                    rhs: Box::new(MathExpression::Number(2.0))
                }),
                rhs: Box::new(MathExpression::Number(3.0))
            }),
            rhs: Box::new(MathExpression::Number(4.0))
        }
    );
    assert_eq!(
        expression_p("1 + 2 * 3 - 4")?,
        MathExpression::BinOp {
            op: ast::BinOp::Subtract,
            lhs: Box::new(MathExpression::BinOp {
                op: ast::BinOp::Add,
                lhs: Box::new(MathExpression::Number(1.0)),
                rhs: Box::new(MathExpression::BinOp {
                    op: ast::BinOp::Multiply,
                    lhs: Box::new(MathExpression::Number(2.0)),
                    rhs: Box::new(MathExpression::Number(3.0))
                })
            }),
            rhs: Box::new(MathExpression::Number(4.0))
        }
    );
    Ok(())
}

#[test]
fn test_interpret() -> Result<(), InterpreterError> {
    assert_eq!(
        interpret(expression_p("1.21").unwrap(), &HashMap::new())?,
        1.21
    );
    assert_eq!(
        interpret(expression_p("x").unwrap(), &HashMap::new()).unwrap_err(),
        InterpreterError::UnboundVariable {
            variable_name: "x".to_string()
        }
    );
    assert_eq!(
        interpret(expression_p("x").unwrap(), &{
            let mut map = HashMap::new();
            map.insert("x".to_string(), 3.14159);
            map
        })?,
        3.14159
    );
    assert_eq!(
        interpret(expression_p("sqrt(x) + sqrt(y)").unwrap(), &{
            let mut map = HashMap::new();
            map.insert("x".to_string(), 196.0);
            map.insert("y".to_string(), 169.0);
            map
        })?,
        27.0
    );
    assert_eq!(
        interpret(expression_p("32 ^ (1/5)").unwrap(), &HashMap::new())?,
        2.0
    );
    assert_eq!(
        interpret(expression_p("32 ^ (1/0)").unwrap(), &HashMap::new()).unwrap_err(),
        InterpreterError::DivideByZero {
            dividend: 1.0,
            divisor: 0.0
        }
    );
    Ok(())
}
