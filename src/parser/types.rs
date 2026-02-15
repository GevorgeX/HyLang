use crate::{errors::syntax_errors::SyntaxError, lexer::token::{TokenInfo, TokenType}};

use super::Parser;

pub enum Type{
    Identifier{token_info: TokenInfo},
    Pointer{token_info: TokenInfo, to: Box<Type>},
}

impl Parser {
    pub fn parse_type(&mut self) -> Result<Type, SyntaxError> {
        self.prefix_type()
    }

    fn prefix_type(&mut self) -> Result<Type, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            match token.token_type {
                TokenType::Star => {
                    self.next_token();
                    let to = self.prefix_type()?;
                    Ok(Type::Pointer { token_info: token.token_info, to: Box::new(to) })
                }
                _ => self.primary_type(),
            }
        } else {
            let token_info = self.get_current_token_info();
            Err(SyntaxError::ExpectedExpression { token_info })
        }
    }

    fn primary_type(&mut self) -> Result<Type, SyntaxError> {
        let token = {
            let token_info = self.get_current_token_info();
            self.next_token().ok_or(SyntaxError::ExpectedExpression { token_info })
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