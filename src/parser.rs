use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{Token, TokenInfo, TokenType};
use crate::parser::declaration::Declaration;

pub mod expression;
pub mod statement;
pub mod declaration;
pub mod types;

pub struct Parser{
    code: Vec<Token>,
    index: usize
}

impl Parser{
    pub fn new(code: Vec<Token>) -> Self {
        Self { code, index: 0}
    }

    pub fn parse(&mut self) -> Result<Option<Declaration>, SyntaxError> {
        self.declaration()
    }

    pub fn get_non_newline(&self) -> usize {
        let mut index = self.index;
        while let Some(token) = self.code.get(index) {
            if token.token_type == TokenType::NewLine {
                index += 1;
            } else {
                break;
            }
        }
        index
    }

    pub fn next_token_newline(&mut self) -> Option<&Token> {
        self.index += 1;
        self.code.get(self.index - 1)
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        let next_non_newline_index = self.get_non_newline();
        if self.index != next_non_newline_index{
            self.index = next_non_newline_index;
        }
        
        let last_index = self.index;
        self.index += 1;
        self.skip_newlines();
        
        self.code.get(last_index) 
     }
    
    pub fn peek_token_newline(&self) -> Option<&Token> {
        self.code.get(self.index)
    }

    pub fn peek_token(&self) -> Option<&Token> {
        let i = self.get_non_newline();
        self.code.get(i)
    }

    pub fn require_token(&mut self, token_type: TokenType) -> Result<&Token, SyntaxError> {
        let i = self.get_non_newline();
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

    pub fn get_current_token_info(&self) -> TokenInfo {
        if self.index >= self.code.len() {
            return TokenInfo{line: 0, index: 0, start: 0, len: 0};
        }
        self.code[self.index].token_info
    }

    pub fn skip_newlines(&mut self) {
        self.index = self.get_non_newline()
    }

}