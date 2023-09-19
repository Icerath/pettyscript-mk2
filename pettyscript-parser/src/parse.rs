use std::str::FromStr;

use crate::expr::{Expr, Literal};

use winnow::ascii::{digit0, digit1};
use winnow::combinator::opt;
use winnow::error::{ErrMode, ErrorKind};
use winnow::prelude::*;
use winnow::{
    ascii::{dec_int, float},
    combinator::alt,
    combinator::cut_err,
    combinator::{delimited, preceded, separated_pair, terminated},
    combinator::{fold_repeat, separated0},
    error::{AddContext, ParserError},
    token::{any, none_of, take, take_while},
};

type In<'a> = &'a str;

pub trait RawErr<'a> = ParserError<In<'a>>;
pub trait CtxErr<'a> = RawErr<'a> + AddContext<In<'a>, &'static str>;

pub fn parse<'a, E: CtxErr<'a>>(mut input: In<'a>) -> PResult<Expr, E> {
    (expr)(&mut input)
}

fn expr<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    preceded(ws, alt((list, literal.map(Expr::Literal)))).parse_next(input)
}

fn list<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    preceded(
        '[',
        cut_err(terminated(
            separated0(expr, (ws, ',', ws)),
            (ws, opt((',', ws)), ']'),
        )),
    )
    .map(|exprs: Vec<Expr>| Expr::List(Box::from(exprs)))
    .context("list")
    .parse_next(input)
}

fn literal<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Literal, E> {
    alt((
        literal_float.map(Literal::Float),
        literal_int.map(Literal::Int),
        literal_string.map(|string| Literal::String(Box::from(string))),
    ))
    .parse_next(input)
}

fn literal_float<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<f64, E> {
    (opt(('-', ws)), digit1, '.', digit0)
        .recognize()
        .map(raw_parse_num::<f64>)
        .parse_next(input)
}

fn literal_int<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<i64, E> {
    (opt(('-', ws)), digit1)
        .recognize()
        .map(raw_parse_num::<i64>)
        .parse_next(input)
}

fn raw_parse_num<T: FromStr + std::ops::Neg<Output = T>>(input: &str) -> T {
    if input.starts_with('-') {
        -input
            .trim_start_matches('-')
            .trim_start()
            .parse::<T>()
            .ok()
            .unwrap()
    } else {
        input.parse().ok().unwrap()
    }
}

fn literal_string<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<String, E> {
    delimited(
        '"',
        fold_repeat(0.., character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        '"',
    )
    .context("string")
    .parse_next(input)
}

/// TODO
fn character<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<char, E> {
    none_of('"').parse_next(input)
}

fn ws<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<&'a str, E> {
    take_while(0.., (' ', '\t', '\r', '\n')).parse_next(input)
}
