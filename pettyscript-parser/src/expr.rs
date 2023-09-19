#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    List(Box<[Expr]>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Box<str>),
}

impl From<Literal> for Expr {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}
