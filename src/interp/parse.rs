use super::ast::{BinOp, MathExpression};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, i64, one_of},
    combinator::{eof, map},
    error::Error,
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated},
    Err, IResult, Parser,
};

pub fn expression_p(input: &str) -> Result<MathExpression, Err<&str>> {
    match terminated(preceded(whitespace_p, terms_p), eof).parse(input) {
        Ok((_, expr)) => Ok(expr),
        Err(err) => Err(err.map(|err| err.input)),
    }
}

fn whitespace_p(input: &str) -> IResult<&str, ()> {
    map(many0(one_of("\n\t\r ")), |_| ()).parse(input)
}

fn operation_chain_p<'a, P, E>(
    operations: &'static str,
    inner_parser: P,
) -> impl Parser<&'a str, Output = MathExpression, Error = Error<&'a str>>
where
    P: Parser<&'a str, Output = MathExpression, Error = Error<&'a str>> + 'a + Copy,
    // E: nom::error::ParseError<&'a str> + 'a,
{
    fn get_op(c: char) -> BinOp {
        match c {
            '*' => BinOp::Multiply,
            '/' => BinOp::Divide,
            '^' => BinOp::Exponent,
            '-' => BinOp::Subtract,
            '+' => BinOp::Add,
            '%' => BinOp::Modulus,
            _ => BinOp::Multiply,
        }
    }

    move |input: &'a str| -> IResult<&'a str, MathExpression, Error<&'a str>> {
        map(
            pair(
                inner_parser,
                many0(pair(
                    terminated(one_of(operations), whitespace_p),
                    inner_parser,
                )),
            ),
            |(first, rest)| {
                rest.into_iter()
                    .fold(first, |acc, (op, rhs)| MathExpression::BinOp {
                        op: get_op(op),
                        lhs: Box::new(acc),
                        rhs: Box::new(rhs),
                    })
            },
        )
        .parse(input)
    }
}

fn variable_p(input: &str) -> IResult<&str, MathExpression> {
    map(terminated(alphanumeric1, whitespace_p), |s: &str| {
        MathExpression::Variable { name: s.into() }
    })
    .parse(input)
}

fn number_p(input: &str) -> IResult<&str, MathExpression> {
    terminated(
        alt((
            map(double, |f| MathExpression::Number(f as f64)),
            map(i64, |f| MathExpression::Number(f as f64)),
        )),
        whitespace_p,
    )
    .parse(input)
}

fn function_p(input: &str) -> IResult<&str, MathExpression> {
    fn function_name(input: &str) -> IResult<&str, String> {
        terminated(
            map(
                alt((
                    tag("sine"),
                    tag("sin"),
                    tag("log"),
                    tag("rand"),
                    tag("sqrt"),
                )),
                String::from,
            ),
            whitespace_p,
        )
        .parse(input)
    }

    map(
        pair(
            function_name,
            delimited(
                terminated(tag("("), whitespace_p),
                separated_list0(terminated(tag(","), whitespace_p), terms_p),
                terminated(tag(")"), whitespace_p),
            ),
        ),
        |(fun, factor)| MathExpression::Function {
            name: fun,
            arguments: factor,
        },
    )
    .parse(input)
}

fn atom_p(input: &str) -> IResult<&str, MathExpression> {
    alt((
        number_p,
        function_p,
        variable_p,
        delimited(
            terminated(tag("("), whitespace_p),
            terms_p,
            terminated(tag(")"), whitespace_p),
        ),
    ))
    .parse(input)
}

fn exponents_p(input: &str) -> IResult<&str, MathExpression, Error<&str>> {
    operation_chain_p::<_, Error<&str>>("^", atom_p).parse(input)
}

fn factor_p(input: &str) -> IResult<&str, MathExpression, Error<&str>> {
    operation_chain_p::<_, Error<&str>>("*/% ", exponents_p).parse(input)
}

fn terms_p(input: &str) -> IResult<&str, MathExpression, Error<&str>> {
    operation_chain_p::<_, Error<&str>>("+-", factor_p).parse(input)
}
