use super::ast::{
    AssignLiteral, Boolean, CallExpression, Expression, Identifier, IntegerLiteral, Program,
    ShapStatement, Statement, StringLiteral,
};
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
            let peek_token = self.peek_token.clone();
            let infix = self.infix_parse(peek_token.clone(), left_expr);

            if !is_infix_operator(&peek_token) {
                return infix;
            }
            left_expr = infix;
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
            Token::STRING(_) => Some(self.parse_string_literal()),
            Token::INT(_) => Some(self.parse_integer_literal()),
            Token::TRUE => Some(self.parse_boolean()),
            Token::FALSE => Some(self.parse_boolean()),
            _ => None,
        }
    }

    fn infix_parse(&mut self, token: Token, left_expression: Expression) -> Expression {
        match token {
            Token::LPAREN => {
                self.next_token();
                self.parse_call_expression(left_expression)
            }
            Token::ASSIGN => {
                self.next_token();
                self.parse_assignment_expression(left_expression)
            }
            _ => left_expression,
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: if let Token::IDENT(s) = self.cur_token.clone() {
                s
            } else {
                String::from("")
            },
        })
    }

    fn parse_string_literal(&mut self) -> Expression {
        let s: StringLiteral = StringLiteral {
            token: self.cur_token.clone(),
            value: if let Token::STRING(s) = self.cur_token.clone() {
                s
            } else {
                String::from("")
            },
        };

        Expression::StringLiteral(s)
    }

    fn parse_integer_literal(&mut self) -> Expression {
        let int = IntegerLiteral {
            token: self.cur_token.clone(),
            value: if let Token::INT(s) = self.cur_token.clone() {
                s
            } else {
                String::from("")
            },
        };

        Expression::IntegerLiteral(int)
    }

    fn parse_boolean(&mut self) -> Expression {
        let b = Boolean {
            token: self.cur_token.clone(),
            value: match self.cur_token {
                Token::TRUE => true.to_string(),
                Token::FALSE => false.to_string(),
                _ => panic!("boolean expected"),
            },
        };

        Expression::Boolean(b)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Expression {
        let call = CallExpression {
            token: self.cur_token.clone(),
            function: Box::new(function),
            arguments: self.parse_expression_list(Token::RPAREN),
        };

        Expression::CallExpression(call)
    }

    fn parse_assignment_expression(&mut self, left_expression: Expression) -> Expression {
        let t = self.cur_token.clone();
        self.next_token();
        let assignment = AssignLiteral {
            token: t,
            name: Box::new(left_expression),
            expression: Box::new(self.parse_expression()),
        };

        Expression::AssignLiteral(assignment)
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
        Token::LBRACKET => true,
        _ => false,
    }
}

#[test]
fn test_parse() {
    let input = r#"#[i18n(zh-CN = "hello", code = 200), Debug]"#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let stmt = p.parse_program().statements;

    assert!(stmt.len() == 1);
}
