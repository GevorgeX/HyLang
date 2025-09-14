use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::Parser;
//use crate::parser::statement::Statement;

pub enum Declaration {
    Namespace{ token_info: TokenInfo, name: TokenInfo, body: Vec<Declaration>, bracket_token: (TokenInfo, TokenInfo) },
    //Function { token_info: TokenInfo, name: TokenInfo, parameters: Vec<TokenInfo>, body: Vec<Statement> },
}

impl Parser {
    pub fn declaration(&mut self) -> Result<Declaration, SyntaxError> {
        while let Some(&token) =  self.peek_token() {
            match token.token_type {
                TokenType::Namespace => {
                    self.next_token();
                    let name_token = self.require_token(TokenType::Ident)?.token_info;
                    let open_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
                    let mut body = Vec::new();

                    while let Some(&token) =  self.peek_token() {
                        if token.token_type == TokenType::RightCBracket{
                            break;
                        }
                        body.push(self.declaration()?);
                    }
                    let close_bracket = self.require_token(TokenType::RightCBracket)?.token_info;
                    return Ok(Declaration::Namespace{ token_info: token.token_info, name: name_token, body, bracket_token: (open_bracket, close_bracket)})
                }
                _ =>  return Err(SyntaxError::UnexpectedToken { token: token.clone() }),
            }
        }

        panic!("Empty File")
    }
}