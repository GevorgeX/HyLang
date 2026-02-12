mod function_define;
pub mod struct_union_define;

use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::declaration::function_define::FunctionArgument;
use crate::parser::expression::block::BlockOfStatements;
use crate::parser::Parser;
use crate::parser::types::Type;

pub enum Declaration {
    Namespace{ token_info: TokenInfo, name: TokenInfo, body: Vec<Declaration>, bracket_token: (TokenInfo, TokenInfo) },
    Function {
        func_token_info: TokenInfo,
        name: TokenInfo,
        parameters: Vec<FunctionArgument>,
        bracket_token: (TokenInfo, TokenInfo),
        return_type: Option<Type>,
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
    pub fn declaration(&mut self) -> Result<Option<Declaration>, SyntaxError> {
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
                        if let Some(decl) = self.declaration()?{
                            body.push(decl);
                        }
                    }
                    let close_bracket = self.require_token(TokenType::RightCBracket)?.token_info;
                    Ok(Some(Declaration::Namespace { token_info: token.token_info, name: name_token, body, bracket_token: (open_bracket, close_bracket)}))
                }
                TokenType::Function => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (lbracket_token_info, parameters, rbracket_token) = self.function_argument()?;
                    let return_type = {
                        if let Some(token) = self.peek_token() {
                            if token.token_type != TokenType::LeftCBracket {
                                Some(self.parse_type()?)
                            }
                            else { None }
                        } else { None }
                    };
                    let body = self.block_of_statements()?;

                    Ok(Some(Declaration::Function {
                        func_token_info: token.token_info,
                        name: name_token_info,
                        parameters,
                        bracket_token: (lbracket_token_info, rbracket_token),
                        return_type,
                        body,
                    }))
                }
                TokenType::Struct => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (open_bracket, fields, close_bracket) = self.struct_union_field()?;
                    Ok(Some(Declaration::Struct {
                        struct_token_info: token.token_info,
                        name: name_token_info,
                        fields,
                        bracket_token: (open_bracket, close_bracket)
                    }))
                }
                TokenType::Union => {
                    self.next_token();
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let (open_bracket, fields, close_bracket) = self.struct_union_field()?;
                    Ok(Some(Declaration::Union {
                        union_token_info: token.token_info,
                        name: name_token_info,
                        fields,
                        bracket_token: (open_bracket, close_bracket)
                    }))
                }
                _ => Err(SyntaxError::UnexpectedToken { token: token.clone() }),
            }
        }

        panic!("Empty File")
    }
}