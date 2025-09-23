use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::Parser;

#[derive(Debug)]
pub struct Field {
    pub(crate) name_token_info: TokenInfo,
    pub(crate) type_token_info: TokenInfo,
}

impl Parser{
    pub fn struct_union_field(&mut self) -> Result<(TokenInfo,Vec<Field>,TokenInfo), SyntaxError> {
        let open_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
        let mut fields = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::RightCBracket {
                break;
            }
            let name_token_info = self.require_token(TokenType::Ident)?.token_info;
            let type_token_info = self.require_token(TokenType::Ident)?.token_info;
            fields.push(Field { name_token_info, type_token_info });
        }
        let close_bracket = self.require_token(TokenType::RightCBracket)?.token_info;
        Ok((open_bracket, fields, close_bracket))
    }
}