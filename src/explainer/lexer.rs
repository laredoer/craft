use std::iter::Peekable;
use std::str::Chars;

use crate::explainer::token;
use crate::explainer::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn peek_is_numeric(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => ch.is_numeric(),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        while self.peek_is_letter() || self.peek_is_numeric() {
            ident.push(self.read_char().unwrap());
        }
        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);
        while let Some(&ch) = self.peek_char() {
            if !ch.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(&ch) = self.peek_char() {
            if ch == '"' {
                // self.read_char();
                break;
            }
            s.push(self.read_char().unwrap());
        }
        self.read_char();
        s
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some(',') => Token::COMMA,
            Some('=') => Token::ASSIGN,
            Some('#') => Token::SHAP,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some('[') => Token::LBRACKET,
            Some(']') => Token::RBRACKET,
            Some('"') => Token::STRING(self.read_string()),
            Some(ch @ _) => {
                if is_letter(ch) {
                    let ident = self.read_identifier(ch);
                    token::lookup_ident(&ident)
                } else if ch.is_numeric() {
                    Token::INT(self.read_number(ch))
                } else {
                    Token::ILLEGAL
                }
            }
            None => Token::EOF,
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || ch == '-'
}

#[test]
fn is_letter_test() {
    assert!(is_letter('_'));
    assert!(is_letter('a'));
    assert!(is_letter('Z'));

    assert!(!is_letter('*'));
    assert!(!is_letter('1'));
}

#[test]
fn next_token_test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = r#"#[i18n(zh-CN="你好 %s", zh-HK="你好", en="hello", code=400, true, false)]"#;
    let tests = vec![
        Token::SHAP,
        Token::LBRACKET,
        Token::IDENT("i18n".to_string()),
        Token::LPAREN,
        Token::IDENT("zh-CN".to_string()),
        Token::ASSIGN,
        Token::STRING("你好 %s".to_string()),
        Token::COMMA,
        Token::IDENT("zh-HK".to_string()),
        Token::ASSIGN,
        Token::STRING("你好".to_string()),
        Token::COMMA,
        Token::IDENT("en".to_string()),
        Token::ASSIGN,
        Token::STRING("hello".to_string()),
        Token::COMMA,
        Token::IDENT("code".to_string()),
        Token::ASSIGN,
        Token::INT("400".to_string()),
        Token::COMMA,
        Token::TRUE,
        Token::COMMA,
        Token::FALSE,
        Token::RPAREN,
        Token::RBRACKET,
        Token::EOF,
    ];

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}
