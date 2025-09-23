mod function_define;
mod struct_union_define;

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
    Struct{
        struct_token_info: TokenInfo,
        name: TokenInfo,
        fields: Vec<struct_union_define::Field>,
        bracket_token: (TokenInfo, TokenInfo)
    },
    Union{
        union_token_info: TokenInfo,
        name: TokenInfo,
        fields: Vec<struct_union_define::Field>,
        bracket_token: (TokenInfo, TokenInfo)
    }
}

impl Parser {
    pub fn declaration(&mut self) -> Result<Declaration, SyntaxError> {
        while let Some(&token) = self.peek_token() {
            return match token.token_type {
                TokenType::Namespace => {
                    self.next_token();
                    let name_token = self.require_token(TokenType::Ident)?.token_info;
                    let open_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
                    let mut body = Vec::new();

                    while let Some(&token) = self.peek_token() {
                        if token.token_type == TokenType::RightCBracket {
                            break;
                        }
                        body.push(self.declaration()?);
                    }
                    let close_bracket = self.require_token(TokenType::RightCBracket)?.token_info;
                    Ok(Declaration::Namespace { token_info: token.token_info, name: name_token, body, bracket_token: (open_bracket, close_bracket) })
                }
                TokenType::Function => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (lbracket_token_info, parameters, rbracket_token) = self.function_argument()?;
                    let type_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let body = self.block_of_statements()?;

                    Ok(Declaration::Function {
                        func_token_info: token.token_info,
                        name: name_token_info,
                        parameters,
                        bracket_token: (lbracket_token_info, rbracket_token),
                        type_token_info,
                        body,
                    })
                }
                TokenType::Struct => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (open_bracket, fields, close_bracket) = self.struct_union_field()?;
                    Ok(Declaration::Struct {
                        struct_token_info: token.token_info,
                        name: name_token_info,
                        fields,
                        bracket_token: (open_bracket, close_bracket)
                    })
                }
                TokenType::Union => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (open_bracket, fields, close_bracket) = self.struct_union_field()?;
                    Ok(Declaration::Union {
                        union_token_info: token.token_info,
                        name: name_token_info,
                        fields,
                        bracket_token: (open_bracket, close_bracket)
                    })
                }
                _ => Err(SyntaxError::UnexpectedToken { token: token.clone() }),
            }
        }

        panic!("Empty File")
    }
}