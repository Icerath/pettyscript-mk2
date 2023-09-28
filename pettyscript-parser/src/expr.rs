#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add(Box<(Expr, Expr)>),
    Sub(Box<(Expr, Expr)>),
    Mul(Box<(Expr, Expr)>),
    Div(Box<(Expr, Expr)>),

    Literal(Literal),
    Ident(Ident),
    Block(Block),
    While {
        condition: Box<Expr>,
        block: Block,
    },
    For {
        ident: Ident,
        iter: Box<Expr>,
        block: Block,
    },
    IfState(IfState),
    Function {
        name: Ident,
        params: Box<[Ident]>,
        body: Block,
    },
    FuncCall(Ident, Box<[Expr]>),
    SetEq(Ident, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfState {
    pub condition: Box<Expr>,
    pub body: Block,
    pub or_else: OrElse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrElse {
    IfState(Box<IfState>),
    Block(Block),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Box<[Expr]>);

#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub Box<str>);

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Vec<Expr>> for Block {
    fn from(value: Vec<Expr>) -> Self {
        Self(value.into())
    }
}

impl From<Ident> for Expr {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Box<str>),
    List(Box<[Expr]>),
}

impl From<Block> for Expr {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
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
