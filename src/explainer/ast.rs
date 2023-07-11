use crate::explainer::token::Token;

pub enum Statements {
    ShapStatement(ShapStatement),
}

pub struct ShapStatement {
    pub token: Token,
    pub elements: Vec<Expressions>,
}

pub enum Expressions {
    Identifier(Identifier),
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}
