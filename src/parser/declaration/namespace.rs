use crate::{errors::syntax_errors::SyntaxError, lexer::token::{TokenInfo, TokenType}, parser::{declaration::Declaration, Parser}};

impl Parser {
    pub fn namespace_body(&mut self) -> Result<(TokenInfo, Vec<Declaration>, TokenInfo), SyntaxError> {
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
        Ok((open_bracket, body, close_bracket))
    }
}