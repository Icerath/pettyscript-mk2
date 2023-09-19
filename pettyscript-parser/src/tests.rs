use winnow::error::ErrorKind;

use crate::expr::{Expr, Literal};
use crate::parse::parse;

macro_rules! parse_eq {
    ($str: expr, $cmp: expr) => {
        assert_eq!(parse::<ErrorKind>($str), Ok($cmp))
    };
}

#[test]
fn test_literal_null() {
    parse_eq!(" null ", Expr::Literal(Literal::Null));
}

#[test]
fn test_literal_bool() {
    parse_eq!(" true ", Expr::Literal(Literal::Bool(true)));
    parse_eq!(" false ", Expr::Literal(Literal::Bool(false)));
}

#[test]
fn test_literal_float() {
    parse_eq!("1.123", Expr::Literal(Literal::Float(1.123)));
    parse_eq!("- 1.123", Expr::Literal(Literal::Float(-1.123)));
}

#[test]
fn test_literal_int() {
    parse_eq!("1", Expr::Literal(Literal::Int(1)));
    parse_eq!("- 42", Expr::Literal(Literal::Int(-42)));
}

#[test]
fn test_list() {
    parse_eq!(
        " [ 1 , 2.5 , [ 2, ], ] ",
        Expr::List(Box::from([
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Float(2.5)),
            Expr::List(Box::from([Expr::Literal(Literal::Int(2))])),
        ]))
    );
}
