use std::fmt::Debug;

use crate::{errors::syntax_errors::SyntaxError, lexer::token::{TokenInfo, TokenType}};

use super::Parser;

pub enum Type{
    Identifier{token_info: TokenInfo},
    Pointer{token_info: TokenInfo, to: Box<Type>},
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Identifier{token_info} => write!(f, "Identifier"),
            Type::Pointer{token_info, to} => write!(f, "Pointer -> {:?}", to),
        }
    }
    
}

impl Parser {
    pub fn parse_type(&mut self) -> Result<Type, SyntaxError> {
        self.prefix_type(false)
    }

    fn prefix_type(&mut self, ignore_newline: bool) -> Result<Type, SyntaxError> {
        if let Some(&token) = self.peek(ignore_newline) {
            match token.token_type {
                TokenType::Star => {
                    self.next(ignore_newline);
                    let to = self.prefix_type(ignore_newline)?;
                    Ok(Type::Pointer { token_info: token.token_info, to: Box::new(to) })
                }
                _ => self.primary_type(ignore_newline),
            }
        } else {
            let token_info = self.get_current_token_info();
            Err(SyntaxError::ExpectedExpression { token_info })
        }
    }

    fn primary_type(&mut self, ignore_newline: bool) -> Result<Type, SyntaxError> {
        let token = {
            let token_info = self.get_current_token_info();
            self.next(ignore_newline).ok_or(SyntaxError::ExpectedExpression { token_info })
        }?.clone();
        match token.token_type {
            TokenType::Ident => Ok(Type::Identifier { token_info: token.token_info }),
            _ => {
                let token_info = token.token_info;
                Err(SyntaxError::ExpectedType { token_info })
            }
        } 
    }
}