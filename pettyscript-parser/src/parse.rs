use crate::expr::{Block, Expr, Ident, Literal};
use std::fmt;
use std::str::FromStr;
use winnow::prelude::*;
use winnow::{
    ascii::{digit0, digit1},
    combinator::{alt, cut_err, opt},
    combinator::{delimited, preceded, terminated},
    combinator::{fold_repeat, separated0},
    error::{AddContext, ParserError},
    token::{none_of, take_while},
};

type In<'a> = &'a str;

pub trait RawErr<'a> = ParserError<In<'a>>;
pub trait CtxErr<'a> = RawErr<'a> + AddContext<In<'a>, &'static str>;

macro_rules! cut_delimiter {
    ($lhs: expr, $middle: expr, $rhs: expr $(,)?) => {
        preceded($lhs, cut_err(terminated($middle, $rhs)))
    };
}

pub fn parse<'a, E: CtxErr<'a>>(mut input: In<'a>) -> PResult<Expr, E> {
    preceded(ws, expr).parse_next(&mut input)
}

fn expr<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    alt((
        block.map(Expr::Block),
        while_loop,
        list,
        literal.map(Expr::Literal),
        ident.map(Expr::Ident),
    ))
    .parse_next(input)
}

fn statement<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    alt((
        block.map(Expr::Block),
        while_loop,
        terminated(list, (ws, ';')),
        terminated(literal.map(Expr::Literal), (ws, ';')),
    ))
    .parse_next(input)
}

fn block<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Block, E> {
    cut_delimiter!(
        ('{', ws),
        fold_repeat(
            0..,
            delimited(ws, statement, ws),
            Vec::new,
            |mut block, expr| {
                block.push(expr);
                block
            }
        ),
        '}'
    )
    .map(Block::from)
    .parse_next(input)
}

fn while_loop<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    let _ = "while".parse_next(input)?;
    let condition = preceded(ws, expr).parse_next(input)?;
    let block = preceded(ws, block).parse_next(input)?;
    Ok(Expr::While {
        condition: Box::new(condition),
        block,
    })
}

fn list<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    cut_delimiter!(
        ('[', ws),
        separated0(expr, (ws, ',', ws)),
        (ws, opt((',', ws)), ']')
    )
    .map(|exprs: Vec<Expr>| Expr::List(Box::from(exprs)))
    .context("list")
    .parse_next(input)
}

fn literal<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Literal, E> {
    alt((
        "null".value(Literal::Null),
        literal_bool.map(Literal::Bool),
        literal_float.map(Literal::Float),
        literal_int.map(Literal::Int),
        literal_string.map(|string| Literal::String(Box::from(string))),
    ))
    .parse_next(input)
}

fn literal_bool<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<bool, E> {
    alt(("true", "false"))
        .map(|str| str == "true")
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

fn raw_parse_num<T>(input: &str) -> T
where
    T: FromStr + std::ops::Neg<Output = T>,
    <T as FromStr>::Err: fmt::Debug,
{
    if !input.starts_with('-') {
        return input.parse().unwrap();
    }
    -input
        .trim_start_matches('-')
        .trim_start()
        .parse::<T>()
        .unwrap()
}

fn literal_string<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<String, E> {
    cut_delimiter!(
        '"',
        fold_repeat(0.., character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        '"'
    )
    .context("string")
    .parse_next(input)
}

fn character<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<char, E> {
    none_of('"').parse_next(input)
}

fn ident<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<Ident, E> {
    take_while(1.., |c| matches!(c, 'a'..='z'|'A'..='Z'|'_'))
        .map(Ident::from)
        .parse_next(input)
}

fn ws<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<&'a str, E> {
    take_while(0.., WHITESPACE).parse_next(input)
}

const WHITESPACE: &[char] = &[' ', '\t', '\r', '\n'];
