pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    BinExp(BinExp),
    Literal(Literal),
}

#[derive(Debug)]
pub struct Assign {
    pub target: Ident,
    pub value: Expr,
}

#[derive(Debug)]
pub struct BinExp {
    pub op: String,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}
