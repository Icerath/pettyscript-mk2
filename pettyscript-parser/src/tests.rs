use winnow::error::ContextError;

use crate::expr::{Expr, Literal};
use crate::parse::parse;

macro_rules! parse_eq {
    ($str: expr, $cmp: expr $(,)?) => {
        assert_eq!(parse::<ContextError<&str>>($str), Ok(Expr::from($cmp)))
    };
}

macro_rules! list {
    ($($expr: expr),* $(,)?) => {
        Expr::List(vec![$( $expr.into(), )*].into_boxed_slice()) }
}

macro_rules! block {
    ($($expr: expr);* $(;)*) => {
        Expr::Block(vec![ $($expr.into(),)* ].into_boxed_slice())

    };
}

#[test]
fn test_literal_null() {
    parse_eq!(" null ", Literal::Null);
}

#[test]
fn test_literal_bool() {
    parse_eq!(" true ", true);
    parse_eq!(" false ", false);
}

#[test]
fn test_literal_float() {
    parse_eq!("1.123", 1.123);
    parse_eq!("- 1.123", -1.123);
}

#[test]
fn test_literal_int() {
    parse_eq!("1", 1);
    parse_eq!("- 42", -42);
}

#[test]
fn test_literal_str() {
    parse_eq!(r#" "Hello, World!" "#, "Hello, World!");
}

#[test]
fn test_list() {
    parse_eq!(" [ 1 , 2.5 , [ 2, ], ] ", list![1, 2.5, list![2]]);
    parse_eq!(" [  ] ", list![]);
}

#[test]
fn test_parse_block() {
    parse_eq!(r#" { }"#, Expr::Block(vec![].into()),);
    parse_eq!(r#" { "hello"; 1; {} }"#, block! { "hello"; 1; block! {}; });
}
