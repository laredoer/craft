use crate::explainer::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    ShapStatement(ShapStatement),
}
#[derive(Debug, PartialEq)]
pub struct ShapStatement {
    pub token: Token,
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    Boolean(Boolean),
    ArrayLiteral(ArrayLiteral),
    CallExpression(CallExpression),
    AssignLiteral(AssignLiteral),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>, // Identifier or FunctionLiteral
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayLiteral {
    pub token: Token, // '['
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct AssignLiteral {
    pub token: Token,                // The '=' token
    pub name: Box<Expression>,       //  name = 'jack', the name Identifier
    pub expression: Box<Expression>, //  name = 'jack', the 'jack' Expression
}

#[derive(Debug, PartialEq)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}
