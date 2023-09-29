use crate::expr::Expr;
use crate::parse::{atom, func_call, ident, CtxErr, In};
use winnow::ascii::space0;
use winnow::combinator::preceded;
use winnow::prelude::*;
use winnow::{
    combinator::delimited,
    combinator::fold_repeat,
    combinator::{alt, cut_err},
};

// Maybe this generates the function signature as well?
// Not sure if that's a good idea.
macro_rules! binop_layer {
    ($input: ident, $next_layer: ident, {
        $($key: literal => $expr: path),* $(,)?
    }) => {{
        let mut init = Some($next_layer.parse_next($input)?);

        fold_repeat(
            0..,
            (alt(( $($key,)*)) , cut_err($next_layer) ),
            move || init.take().unwrap(),
            |acc, (op, val): (_, Expr)| match op {
                $($key => $expr(Box::new((acc, val))),)*
                _ => unreachable!(),
            },
        )
    }};
}

#[inline]
pub fn bin_expr<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    comparison(input)
}

fn comparison<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    binop_layer!(input, plus_minus, {
        "==" => Expr::EqEq,
        "||" => Expr::Or,
        "&&" => Expr::And,
    })
    .context("comparison")
    .parse_next(input)
}

pub fn plus_minus<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    binop_layer!(input, mul_div, {
        '+' => Expr::Add,
        '-' => Expr::Sub,
    })
    .context("bin_expr")
    .parse_next(input)
}

fn mul_div<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    binop_layer!(input, get_item, {
        '*' => Expr::Mul,
        '/' => Expr::Div,
    })
    .context("mul_div")
    .parse_next(input)
}

fn get_item<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    let mut init = Some(factor.parse_next(input)?);
    fold_repeat(
        0..,
        preceded('.', cut_err(alt((func_call, ident.map(Expr::Ident))))),
        move || init.take().unwrap(),
        |acc, val: Expr| match val {
            Expr::Ident(ident) => Expr::GetItem(Box::new(acc), ident),
            Expr::FuncCall(ident, args) => Expr::MethodCall(Box::new(acc), ident, args),
            _ => unreachable!(),
        },
    )
    .context("get_item")
    .parse_next(input)
}

fn factor<'a, E: CtxErr<'a>>(i: &mut In<'a>) -> PResult<Expr, E> {
    delimited(space0, alt((atom, delimited('(', bin_expr, ')'))), space0)
        .context("factor")
        .parse_next(i)
}
