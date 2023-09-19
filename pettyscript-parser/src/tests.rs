use winnow::error::ErrorKind;

use crate::expr::{Expr, Literal};
use crate::parse::parse;

macro_rules! parse_eq {
    ($str: expr, $cmp: expr) => {
        assert_eq!(parse::<ErrorKind>($str), Ok($cmp.into()))
    };
}

macro_rules! list {
    ($($expr: expr),*) => {
        Expr::List(Box::from( [ $( $expr.into(), )* ])) };
}

#[test]
fn test_literal_null() {
    parse_eq!(" null ", Literal::Null);
}

#[test]
fn test_literal_bool() {
    parse_eq!(" true ", Literal::Bool(true));
    parse_eq!(" false ", Literal::Bool(false));
}

#[test]
fn test_literal_float() {
    parse_eq!("1.123", Literal::Float(1.123));
    parse_eq!("- 1.123", Literal::Float(-1.123));
}

#[test]
fn test_literal_int() {
    parse_eq!("1", Literal::Int(1));
    parse_eq!("- 42", Literal::Int(-42));
}

#[test]
fn test_list() {
    parse_eq!(
        " [ 1 , 2.5 , [ 2, ], ] ",
        list![Literal::Int(1), Literal::Float(2.5), list![Literal::Int(2)]]
    );
}
