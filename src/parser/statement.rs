use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{Token, TokenInfo, TokenType};
use crate::parser::expression::Expression;
use crate::parser::Parser;

pub enum Statement {
    ExpressionStatement(Expression),
    DefineVariable{identifier: Token, value: Option<(TokenInfo, Expression)>, token_info: TokenInfo},
}

impl Parser {
    pub fn statement(&mut self) -> Result<Statement, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            match token.token_type {
                TokenType::Const | TokenType::Var => {
                    self.next_token();
                    let identifier = self.require_token(TokenType::Ident)?.clone();
                    let mut value = None;
                    if let Some(&equal_token) = self.peek_token(){
                        if equal_token.token_type == TokenType::Equal {
                            self.next_token();
                            let expr = self.expression()?;
                            value = Some((equal_token.token_info, expr));
                        }
                    }
                    return Ok(Statement::DefineVariable {
                        identifier,
                        value,
                        token_info: token.token_info,
                    })
                }
                _ => ()
            }
        }
        let expr = self.expression()?;
        Ok(Statement::ExpressionStatement(expr))
    }
}