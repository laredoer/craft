use super::ast::{CallExpression, Expression, Identifier, Program, ShapStatement, Statement};
use super::lexer::Lexer;
use super::token::Token;

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'_>) -> Parser<'_> {
        let mut l = l;
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        Parser {
            l,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                statements.push(s);
            }
            self.next_token();
        }

        Program {
            statements: statements,
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::SHAP => Some(Statement::ShapStatement(self.parse_shap_statement())),
            _ => None,
        }
    }

    fn parse_shap_statement(&mut self) -> ShapStatement {
        let mut stmt = ShapStatement {
            token: self.cur_token.clone(),
            elements: vec![],
        };
        stmt.token = self.cur_token.clone();
        self.next_token();
        stmt.elements = self.parse_expression_list(Token::RBRACKET);

        stmt
    }

    fn parse_expression(&mut self) -> Expression {
        let mut left_expr = self.prefix_parse(self.cur_token.clone()).unwrap();

        while !self.peek_token_is(Token::SEMICOLON) {
            let t = self.peek_token.clone();
            self.next_token();
            let infix_expression = self.infix_parse(&t, left_expr);

            if !is_infix_operator(&t) {
                return infix_expression;
            }

            left_expr = infix_expression;
        }

        left_expr
    }

    fn parse_expression_list(&mut self, end_token: Token) -> Vec<Expression> {
        let mut list: Vec<Expression> = vec![];

        if self.peek_token_is(end_token.clone()) {
            self.next_token();
            return list;
        }

        self.next_token();
        list.push(self.parse_expression());

        while self.peek_token_is(Token::COMMA) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression())
        }

        self.expect_peek_token(end_token.clone());

        list
    }

    fn prefix_parse(&mut self, token: Token) -> Option<Expression> {
        match token {
            Token::IDENT(_) => Some(self.parse_identifier()),
            _ => None,
        }
    }

    fn infix_parse(&mut self, token: &Token, left_expression: Expression) -> Expression {
        match token {
            Token::LPAREN => self.parse_call_expression(left_expression),
            _ => left_expression,
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: String::from(""),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Expression {
        let mut call = CallExpression {
            token: self.cur_token.clone(),
            function: Box::new(function),
            arguments: vec![],
        };
        call.arguments = self.parse_expression_list(Token::RPAREN);

        Expression::CallExpression(call)
    }

    fn peek_token_is(&self, token: Token) -> bool {
        match self.peek_token {
            Token::EOF => false,
            _ => self.peek_token == token,
        }
    }

    fn expect_peek_token(&mut self, t: Token) -> bool {
        match self.peek_token_is(t) {
            true => {
                self.next_token();
                true
            }
            false => false,
        }
    }
}

fn is_infix_operator(t: &Token) -> bool {
    match t {
        Token::RBRACKET | Token::LBRACKET => true,
        _ => false,
    }
}

#[test]
fn test_parse() {
    let input = r#"#[i18n(), debug]; #[clone]"#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    assert!(p.parse_program().statements.len() == 2);
}
