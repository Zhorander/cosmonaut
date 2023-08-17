#[derive(Clone, Debug)]
pub enum Pattern {
    Identifier(String),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    Unit,
    Lambda(Pattern, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
    LetBinding(Pattern, Box<Expression>, Option<Box<Expression>>),
}
