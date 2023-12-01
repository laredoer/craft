use crate::explainer::token::Token;

use super::extension::{Extension, Field};

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

impl Expression {
    pub fn to_extension(&self) -> Option<Extension> {
        match self {
            Expression::Identifier(i) => Some(Extension {
                name: i.value.clone(),
                ..Default::default()
            }),
            Expression::CallExpression(c) => {
                let mut call = Extension {
                    ..Default::default()
                };

                if let Expression::Identifier(i) = c.function.as_ref() {
                    call.name = i.value.clone();
                }

                let mut args = vec![];
                for arg in &c.arguments {
                    match arg {
                        Expression::AssignLiteral(i) => {
                            let field_name = match i.name.as_ref() {
                                Expression::Identifier(i) => i.value.clone(),
                                _ => "".to_string(),
                            };

                            let value = match i.expression.as_ref() {
                                Expression::StringLiteral(s) => {
                                    Some((s.value.clone(), "string".to_string()))
                                }
                                Expression::IntegerLiteral(i) => {
                                    Some((i.value.clone(), "int".to_string()))
                                }
                                Expression::Boolean(b) => {
                                    Some((b.value.clone(), "bool".to_string()))
                                }
                                _ => None,
                            }
                            .unwrap();

                            args.push(Field {
                                name: field_name,
                                go_type: value.1,
                                value: value.0,
                            });
                        }
                        Expression::Identifier(ident) => {
                            args.push(Field {
                                name: ident.value.clone(),
                                go_type: "string".to_string(),
                                value: "".to_string(),
                            });
                        }
                        _ => continue,
                    }
                }
                call.args = Some(args);
                Some(call)
            }
            _ => None,
        }
    }
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
    pub value: String,
}
