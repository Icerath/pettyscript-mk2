use winnow::error::ContextError;

use crate::expr::{Block, Expr, Literal};
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
        Block(vec![ $($expr.into(),)* ].into_boxed_slice())
    };
}

macro_rules! while_ {
    ($condition: expr => { $($expr: expr);* $(;)* } ) => {
        Expr::While {
            condition: Box::new($condition.into()),
            block: block! { $($expr;)* },
        }
    };
}

macro_rules! for_ {
    ($ident: ident in $iter: expr => { $($expr: expr);* $(;)* } ) => {
        Expr::For {
            ident: stringify!($ident).into(),
            iter: $iter.into(),
            block: block! { $($expr;)* },
        }
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
    parse_eq!(" [  ] ", list![]);
    parse_eq!(" [ 1 , 2.5 , [ 2 , ] , ] ", list![1, 2.5, list![2]]);
}

#[test]
fn test_parse_block() {
    parse_eq!(r#" { }"#, block! {});
    parse_eq!(r#" { "hello"; 1; {} }"#, block! { "hello"; 1; block! {}; });
}

#[test]
fn test_while_loop() {
    parse_eq!(r#" while true { }"#, while_! { true => {} });
    parse_eq!(
        r#" while false { "hello"; 1; }"#,
        while_! { false => { "hello"; 1; } }
    );
}

#[test]
fn test_for_loop() {
    parse_eq!(
        r#" for i in [1, 2, 3] {}"#,
        for_! { i in list![1, 2, 3] => {} }
    );

    parse_eq!(
        r#" for i in iter { 1; [2, 3]; }"#,
        for_! { i in Expr::Ident("iter".into()) => {
            1;
            list![2, 3];
        }}
    );
}

#[test]
fn test_ident() {
    parse_eq!(r#" hello_world "#, Expr::Ident("hello_world".into()));
}
