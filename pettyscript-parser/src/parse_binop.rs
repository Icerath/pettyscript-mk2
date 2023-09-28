use crate::expr::Expr;
use crate::parse::{atom, CtxErr, In};
use winnow::ascii::space0;
use winnow::prelude::*;
use winnow::token::one_of;
use winnow::{
    combinator::delimited,
    combinator::fold_repeat,
    combinator::{alt, cut_err},
};

pub fn bin_expr<'a, E: CtxErr<'a>>(i: &mut In<'a>) -> PResult<Expr, E> {
    let mut init = Some(term.parse_next(i)?);

    fold_repeat(
        0..,
        (one_of(['+', '-']), cut_err(term)),
        move || init.take().unwrap(),
        |acc, (op, val): (char, Expr)| match op {
            '+' => Expr::Add(Box::new((acc, val))),
            '-' => Expr::Sub(Box::new((acc, val))),
            _ => unreachable!(),
        },
    )
    .context("bin_expr")
    .parse_next(i)
}

fn term<'a, E: CtxErr<'a>>(i: &mut In<'a>) -> PResult<Expr, E> {
    let mut init = Some(factor.parse_next(i)?);

    fold_repeat(
        0..,
        (one_of(['*', '/']), factor),
        move || init.take().unwrap(),
        |acc, (op, val): (char, Expr)| {
            if op == '*' {
                Expr::Mul(Box::new((acc, val)))
            } else {
                Expr::Div(Box::new((acc, val)))
            }
        },
    )
    .context("term")
    .parse_next(i)
}

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
fn factor<'a, E: CtxErr<'a>>(i: &mut In<'a>) -> PResult<Expr, E> {
    delimited(space0, alt((atom, delimited('(', bin_expr, ')'))), space0)
        .context("factor")
        .parse_next(i)
}
