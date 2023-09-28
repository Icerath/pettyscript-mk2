use crate::expr::{Block, Expr, Ident, IfState, Literal, OrElse};
use crate::parse_binop;
use std::fmt;
use std::str::FromStr;
use winnow::combinator::fail;
use winnow::error::ErrMode;
use winnow::prelude::*;
use winnow::token::any;
use winnow::{
    ascii::{digit0, digit1},
    combinator::{alt, cut_err, opt},
    combinator::{delimited, preceded, terminated},
    combinator::{fold_repeat, separated0},
    error::{AddContext, ParserError},
    token::{none_of, take_while},
};

pub type In<'a> = &'a str;

pub trait RawErr<'a> = ParserError<In<'a>> + fmt::Debug;
pub trait CtxErr<'a> = RawErr<'a> + AddContext<In<'a>, &'static str>;

macro_rules! cut_delimiter {
    ($lhs: expr, $middle: expr, $rhs: expr $(,)?) => {
        preceded($lhs, cut_err(terminated($middle, $rhs)))
    };
}

pub fn parse<'a, E: CtxErr<'a>>(mut input: In<'a>) -> PResult<Expr, E> {
    preceded(ws, statement).parse_next(&mut input)
}

pub(crate) fn atom<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    alt((
        literal.map(Expr::Literal),
        fn_call,
        ident.map(Expr::Ident),
        fail.context("atom"),
    ))
    .parse_next(input)
}

pub(crate) fn expr<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    parse_binop::bin_expr.parse_next(input)
}

fn statement<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    alt((
        block.map(Expr::Block),
        while_loop,
        for_loop,
        if_statement.map(Expr::IfState),
        function_def,
        set_eq,
        terminated(expr, (ws, ';').context("semicolon")),
        fail.context("statement"),
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
    .context("block")
    .map(Block::from)
    .parse_next(input)
}

fn while_loop<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    let _ = ("while").parse_next(input)?;
    cut_err((preceded(ws, expr), preceded(ws, block)))
        .context("while loop")
        .parse_next(input)
        .map(|(condition, block)| Expr::While {
            condition: Box::new(condition),
            block,
        })
}

fn for_loop<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    let _ = ("for", ws).parse_next(input)?;
    cut_err((ident, preceded((ws, "in", ws), expr), preceded(ws, block)))
        .context("for loop")
        .parse_next(input)
        .map(|(ident, iter, block)| Expr::For {
            ident,
            iter: Box::new(iter),
            block,
        })
}

fn if_statement<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<IfState, E> {
    let (condition, body) = preceded("if", cut_err((delimited(ws, expr, ws), block)))
        .context("if statement")
        .parse_next(input)?;

    let or_else: Result<_, ErrMode<E>> = (ws, "else", ws).parse_next(input);

    let or_else = match or_else {
        Ok(_) => match if_statement::<E>(input) {
            Ok(if_state) => OrElse::IfState(Box::new(if_state)),
            Err(_) => OrElse::Block(block(input)?),
        },
        Err(_) => OrElse::None,
    };

    Ok(IfState {
        condition: Box::new(condition),
        body,
        or_else,
    })
}

fn function_def<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    preceded(
        terminated("fn", ws),
        cut_err((
            terminated(ident, ws),
            delimited(
                ('(', ws),
                separated0(ident, (ws, ',', ws)),
                (ws, opt(','), ws, ')'),
            )
            .context("function params"),
            preceded(ws, block),
        )),
    )
    .context("function def")
    .map(|(name, params, body)| Expr::Function {
        name,
        params: Vec::into(params),
        body,
    })
    .parse_next(input)
}

fn set_eq<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    terminated((ident, delimited(ws, '=', ws), expr), ';')
        .map(|(ident, _, expr)| Expr::SetEq(ident, Box::new(expr)))
        .parse_next(input)
}

fn literal<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Literal, E> {
    alt((
        "null".value(Literal::Null),
        literal_bool.map(Literal::Bool),
        literal_float.map(Literal::Float),
        literal_int.map(Literal::Int),
        literal_string.map(|string| Literal::String(Box::from(string))),
        literal_list,
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
    fn character<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<char, E> {
        none_of('"').parse_next(input)
    }
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

fn literal_list<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Literal, E> {
    cut_delimiter!(
        ('[', ws),
        separated0(expr, (ws, ',', ws)),
        (ws, opt((',', ws)), ']')
    )
    .map(|exprs: Vec<Expr>| Literal::List(Box::from(exprs)))
    .context("list")
    .parse_next(input)
}

fn fn_call<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Expr, E> {
    (ident, ws, fn_args)
        .map(|(ident, _, args)| Expr::FuncCall(ident, args))
        .context("fn_call")
        .parse_next(input)
}

fn fn_args<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Box<[Expr]>, E> {
    cut_delimiter!(
        ('(', ws),
        separated0(expr, (ws, ',', ws)),
        (ws, opt((',', ws)), ')')
    )
    .map(|exprs: Vec<Expr>| Box::from(exprs))
    .context("fn_args")
    .parse_next(input)
}

fn ident<'a, E: CtxErr<'a>>(input: &mut In<'a>) -> PResult<Ident, E> {
    (
        any.verify(|c: &char| c.is_ascii_alphabetic() || *c == '_'),
        take_while(0.., |c: char| c.is_ascii_alphanumeric() || c == '_'),
    )
        .context("ident")
        .recognize()
        .map(Ident::from)
        .parse_next(input)
}

fn ws<'a, E: RawErr<'a>>(input: &mut In<'a>) -> PResult<&'a str, E> {
    take_while(0.., WHITESPACE).parse_next(input)
}

const WHITESPACE: &[char] = &[' ', '\t', '\r', '\n'];
