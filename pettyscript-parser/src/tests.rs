use crate::expr::{Block, Expr, Ident, IfState, Literal, OrElse};
use crate::parse::parse;
use winnow::error::ContextError;

macro_rules! parse_eq {
    ($str: expr, $cmp: expr $(,)?) => {
        assert_eq!(parse::<ContextError<&str>>($str), Ok(Expr::from($cmp)))
    };
}

macro_rules! list {
    ($($expr: expr),* $(,)?) => {
        Literal::List(vec![$( $expr.into(), )*].into_boxed_slice()) }
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
            iter: Box::new($iter.into()),
            block: block! { $($expr;)* },
        }
    };
}

macro_rules! ident {
    ($ident: ident) => {
        Ident::from(stringify!($ident))
    };
}

macro_rules! if_ {
    ($condition: expr => { $($expr: expr);* $(;)* }) => {
        Expr::IfState(IfState {
            condition: Box::new($condition.into()),
            body: block! { $($expr;)* },
            or_else: OrElse::None,
        })
    };
    ($condition: expr => { $($expr: expr);* $(;)* } else { $($else_expr: expr);* $(;)* } $(,)? ) => {
        Expr::IfState(IfState {
            condition: Box::new($condition.into()),
            body: block! { $($expr;)* },
            or_else: OrElse::Block(block! { $($else_expr;)* }),
        })
    };
}

macro_rules! set_eq {
    ($ident: ident = $value: expr) => {
        Expr::SetEq(ident!($ident), Box::new($value.into()))
    };
}

macro_rules! fn_ {
    ($name: ident ( $($arg: ident),* ) $(,)* { $($expr: expr);* $(;)* } ) => {
        Expr::Function {
            name: ident!($name),
            params: vec![$(ident!($arg),)* ].into(),
            body: block! { $($expr;)* }
        }
    };
}

macro_rules! call {
    ($ident: ident ($($expr:expr),* $(,)?)) => {
        Expr::FuncCall(ident!($ident), vec![$($expr.into(),)*].into_boxed_slice())
    };
}

#[test]
fn test_literal_null() {
    parse_eq!(" null ; ", Literal::Null);
}

#[test]
fn test_literal_bool() {
    parse_eq!(" true ;", true);
    parse_eq!(" false ; ", false);
}

#[test]
fn test_literal_float() {
    parse_eq!("1.123 ;", 1.123);
    parse_eq!("- 1.123 ;", -1.123);
}

#[test]
fn test_literal_int() {
    parse_eq!("1 ;", 1);
    parse_eq!("- 42 ;", -42);
}

#[test]
fn test_literal_str() {
    parse_eq!(r#" "Hello, World!"; "#, "Hello, World!");
}

#[test]
fn test_list() {
    parse_eq!(" [  ] ; ", list![]);
    parse_eq!(" [ 1 , 2.5 , [ 2 , ] , ] ; ", list![1, 2.5, list![2]]);
}

#[test]
fn test_parse_block() {
    parse_eq!(r#" { }; "#, block! {});
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
        for_! { i in ident!(iter) => {
            1;
            list![2, 3];
        }}
    );
}

#[test]
fn test_ident() {
    parse_eq!(r#" hello_world ; "#, ident!(hello_world),);
    parse_eq!(r#" _asdF1; "#, ident!(_asdF1));
}

#[test]
fn test_if_statement() {
    parse_eq!(r#"if true { "hello"; }"#, if_! { true => { "hello"; } });
    parse_eq!(
        r#"if false { "hello"; } else { "goodbye"; }"#,
        if_! { false => { "hello"; } else { "goodbye"; } }
    );
}

#[test]
fn test_fn_def() {
    parse_eq!(
        r#"fn func (arg1, arg2) { "hello"; } "#,
        fn_!(func (arg1, arg2) { "hello"; })
    );
}

#[test]
fn test_set_eq() {
    parse_eq!(r#"hello = 10;"#, set_eq!(hello = 10));
    parse_eq!(r#"hello = "hello";"#, set_eq!(hello = "hello"));
    parse_eq!(
        r#"hello = [1, 2, bye];"#,
        set_eq!(hello = list![1, 2, ident!(bye)])
    );
}

#[test]
fn test_fn_call() {
    parse_eq!("foo();", call!(foo()));
    parse_eq!("bar(1, 2);", call!(bar(1, 2)));
    parse_eq!("baz(foo());", call!(baz(call!(foo()))));
}
