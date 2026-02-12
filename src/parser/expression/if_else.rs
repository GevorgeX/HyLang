use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::expression::block::BlockOfStatements;
use crate::parser::expression::Expression;
use crate::parser::Parser;

pub struct IfElseBranch {
    pub(crate) condition: Expression,
    pub(crate) body: BlockOfStatements,
    pub(crate) token_info: TokenInfo,
}

pub struct ElseBranch {
    pub(crate) body: BlockOfStatements,
    pub(crate) token_info: TokenInfo,
}

impl Parser{
    fn if_block(&mut self, if_token: TokenInfo) -> Result<IfElseBranch, SyntaxError> {
        let condition = self.expression()?;
        let if_body= self.block_of_statements()?;
        Ok(IfElseBranch {
            condition,
            body: if_body,
            token_info: if_token,
        })
    }

    fn elif_block(&mut self, elif_token: TokenInfo) -> Result<IfElseBranch, SyntaxError> {
        let condition = self.expression()?;
        let elif_body = self.block_of_statements()?;
        Ok(IfElseBranch {
            condition,
            body: elif_body,
            token_info: elif_token,
        })
    }

    fn else_block(&mut self, else_token: TokenInfo) -> Result<ElseBranch, SyntaxError> {
        let else_body = self.block_of_statements()?;
        Ok(ElseBranch {
            body: else_body,
            token_info: else_token,
        })
    }
    pub(crate) fn if_else(&mut self, if_token: TokenInfo) -> Result<Expression, SyntaxError> {
        let if_block = self.if_block(if_token)?;

        let mut else_if_blocks = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::Elif {
                self.next_token();
                let elif_token = token.token_info;
                let elif = self.elif_block(elif_token)?;
                else_if_blocks.push(elif);
            } else {
                break;
            }
        }

        let else_block = if let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::Else {
                self.next_token();
                let else_token = token.token_info;
                Some(self.else_block(else_token)?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(Expression::IfElse {
            if_block: Box::new(if_block),
            else_if_blocks: if else_if_blocks.is_empty() { None } else { Some(else_if_blocks) },
            else_block,
        })
    }
}
