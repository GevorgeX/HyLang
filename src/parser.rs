use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{Token, TokenInfo, TokenType};
use crate::parser::declaration::Declaration;

pub mod expression;
pub mod statement;
pub mod declaration;
pub mod types;

pub struct Parser{
    code: Vec<Token>,
    index: usize,
}

impl Parser{
    pub fn new(code: Vec<Token>) -> Self {
        Self { code, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Option<Declaration>, SyntaxError> {
        self.declaration()
    }

    fn skip_new_lines(&self) -> usize {
        let mut i = self.index;
        while let Some(token) = self.code.get(i) {
            if token.token_type == TokenType::NewLine {
                i += 1;
            } else {
                break;
            }
        }
        i
    }

    pub fn next_token_with_nl(&mut self) -> Option<&Token> {
        self.index += 1;
        self.code.get(self.index - 1)
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        self.index = self.skip_new_lines() + 1;
        self.code.get(self.index - 1)
    }

    pub fn require_token(&mut self, token_type: TokenType) -> Result<&Token, SyntaxError> {
        let i = self.skip_new_lines();
        let token = self.code.get(i);
        if let Some(t) = token {
            if t.token_type == token_type{
                self.index = i + 1;
                return Ok(t)
            }
        }
        let token_info = self.get_current_token_info();
        Err(SyntaxError::ExpectedToken {token_info, expected: token_type})
    }
    

    pub fn peek_token(&self) -> Option<&Token> {
        let i = self.skip_new_lines();
        self.code.get(i)
    }

    pub fn peek_token_with_newline(&self) -> Option<&Token> {
        self.code.get(self.index)
    }

    pub fn get_current_token_info(&self) -> TokenInfo {
        if self.index >= self.code.len() {
            return TokenInfo{line: 0, index: 0, start: 0, len: 0};
        }
        self.code[self.index].token_info
    }

    fn next(&mut self, ignore_newline: bool) -> Option<&crate::lexer::token::Token> {
        if ignore_newline {
            self.next_token()
        } else {
            self.next_token_with_nl()
        }
    }

    fn peek(&self, ignore_newline: bool) -> Option<&crate::lexer::token::Token> {
        if ignore_newline {
            self.peek_token()
        } else {
            self.peek_token_with_newline()
        }
    }
}