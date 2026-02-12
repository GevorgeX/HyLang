use crate::{errors::syntax_errors::SyntaxError, lexer::token::TokenInfo, parser::{expression::Expression, Parser}};

impl Parser{
    pub(crate) fn while_exp(&mut self, token_info: TokenInfo) -> Result<Expression, SyntaxError> {
        let exp = self.expression()?;
        let body = self.block_of_statements()?;
        Ok(Expression::While {
            condition: Box::new(exp),
            body,
            token_info
        })
    }
}