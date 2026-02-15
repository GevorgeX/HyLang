use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::types::Type;
use crate::parser::Parser;

pub struct Field {
    pub name_token_info: TokenInfo,
    pub field_type: Type,
}

impl Parser{
        fn require_field_end(&mut self) -> Result<(), SyntaxError> {
        let mut found = false;
        while let Some(token) = self.peek_token_non_ignore_nl() {
            match token.token_type {
                TokenType::RightCBracket => {
                    found = true;
                    break;
                }
                TokenType::NewLine | TokenType::Comma => {
                    self.index += 1;
                    found = true;
                }
                _ => break,
            }
        }

        if !found {

            let token_info = self.get_current_token_info();
            return Err(SyntaxError::ExpectedToken { token_info, expected: TokenType::DotComma });
        }
        Ok(())
    }

    pub fn struct_union_field(&mut self) -> Result<(TokenInfo,Vec<Field>,TokenInfo), SyntaxError> {
        let open_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
        let mut fields = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::RightCBracket {
                break;
            }
            let name_token_info = self.require_token(TokenType::Ident)?.token_info;
            let field_type = self.parse_type()?;
            fields.push(Field { name_token_info, field_type });
            self.require_field_end()?;
        }
        let close_bracket = self.require_token(TokenType::RightCBracket)?.token_info;
        Ok((open_bracket, fields, close_bracket))
    }
}