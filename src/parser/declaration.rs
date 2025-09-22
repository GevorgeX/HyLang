mod function_define;

use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::declaration::function_define::FunctionArgument;
use crate::parser::expression::control_flow::BlockOfStatements;
use crate::parser::Parser;

pub enum Declaration {
    Namespace{ token_info: TokenInfo, name: TokenInfo, body: Vec<Declaration>, bracket_token: (TokenInfo, TokenInfo) },
    Function {
        func_token_info: TokenInfo,
        name: TokenInfo,
        parameters: Vec<FunctionArgument>,
        bracket_token: (TokenInfo, TokenInfo),
        type_token_info: TokenInfo,
        body: BlockOfStatements
    },
}

impl Parser {
    pub fn declaration(&mut self) -> Result<Declaration, SyntaxError> {
        while let Some(&token) = self.peek_token() {
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
                TokenType::Function => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let lbracket_token_info = self.require_token(TokenType::LeftRBracket)?.clone();
                    let (parameters, rbracket_token) = self.function_argument()?;
                    let type_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let body = self.block_of_statements()?;

                    return Ok(Declaration::Function {
                        func_token_info: token.token_info,
                        name: name_token_info,
                        parameters,
                        bracket_token: (lbracket_token_info.token_info, rbracket_token),
                        type_token_info,
                        body,
                    });
                }
                _ =>  return Err(SyntaxError::UnexpectedToken { token: token.clone() }),
            }
        }

        panic!("Empty File")
    }
}