use crate::{errors::syntax_errors::SyntaxError, lexer::token::{TokenInfo, TokenType}, parser::{expression::Expression, types::Type, Parser}};

impl Parser {
    pub fn var_initialization(&mut self) -> Result<Option<(TokenInfo, Expression)>, SyntaxError> {
        let mut value = None;
        if let Some(&equal_token) = self.peek_token() {
            if equal_token.token_type == TokenType::Equal {
                self.next_token();
                let expr = self.expression_newline()?;
                value = Some((equal_token.token_info, expr));
            }
        }
        Ok(value)
    }

    pub fn define_var_type(&mut self) -> Result<Option<Type>, SyntaxError> {
        let mut var_type = None;
        if !self.is_variable_decl_end() {
            var_type = Some(self.parse_type()?);
        }
        Ok(var_type)
    }
}