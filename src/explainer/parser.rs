use crate::explainer::lexer::Lexer;
use crate::explainer::token::Token;
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


    pub fn parse_program(&mut self) {
        

    }

}
