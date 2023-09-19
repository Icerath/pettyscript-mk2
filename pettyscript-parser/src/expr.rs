#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    List(Box<[Expr]>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(Box<str>),
    Array(Box<[Expr]>),
}
