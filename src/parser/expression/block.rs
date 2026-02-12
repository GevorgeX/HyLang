use crate::{errors::syntax_errors::SyntaxError, lexer::token::{TokenInfo, TokenType}, parser::{expression::Expression, statement::Statement, Parser}};

pub struct BlockOfStatements {
    pub(crate) statements: Vec<Statement>,
    pub(crate) brackets_token_info: (TokenInfo, TokenInfo)
}

impl Parser{
    pub(crate) fn block(&mut self) -> Result<Expression, SyntaxError> {
        self.index -= 1;
        let body = self.block_of_statements()?;
        Ok(Expression::Block {
            body,
        })
    }

    pub fn block_of_statements(&mut self) -> Result<BlockOfStatements, SyntaxError> {
        let left_bracket = self.require_token(TokenType::LeftCBracket)?.clone();
        let mut body = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::RightCBracket {
                break;
            }
            let stmt = self.statement()?;
            body.push(stmt);
        }
        let right_bracket = self.require_token(TokenType::RightCBracket)?;
        Ok(BlockOfStatements {
            statements: body,
            brackets_token_info: (left_bracket.token_info, right_bracket.token_info),
        })
    }

}