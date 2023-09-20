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

impl<T: Into<Literal>> From<T> for Expr {
    fn from(value: T) -> Expr {
        Expr::Literal(value.into())
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value.into_boxed_str())
    }
}
impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
