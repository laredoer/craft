use crate::explainer::{lexer::Lexer, parser::Parser};

use super::ast::{Expression, Statement};

#[derive(Debug, Default)]
pub struct Extension {
    pub name: String,
    pub args: Option<Vec<Field>>,
}

#[derive(Debug, Default)]
pub struct Field {
    pub name: String,
    pub go_type: String,
    pub value: String,
}

pub fn parse_extension(input: String) -> Vec<Extension> {
    let l = Lexer::new(&input);
    let mut p = Parser::new(l);

    let stmt = p.parse_program().statements;

    let exprs = stmt
        .into_iter()
        .map(|s| match s {
            Statement::ShapStatement(ss) => ss.elements,
        })
        .flatten()
        .collect::<Vec<Expression>>();

    let mut res = vec![];
    for expr in &exprs {
        res.push(expr.to_extension().unwrap())
    }

    res
}
