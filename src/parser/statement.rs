use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{Token, TokenInfo, TokenType};
use crate::parser::expression::Expression;
use crate::parser::Parser;

pub enum Statement {
    ExpressionStatement(Expression),
    DefineVariable{identifier: TokenInfo, value: Option<(TokenInfo, Expression)>, token_info: TokenInfo},
    DefineConstantVariable{identifier: TokenInfo, value: Option<(TokenInfo, Expression)>, token_info: TokenInfo},
}

impl Parser {
    fn require_statement_end(&mut self) -> Result<(), SyntaxError> {
        let mut found = false;
        while let Some(token) = self.peek_token_with_newline() {
            match token.token_type {
                TokenType::RightCBracket => {
                    found = true;
                    break;
                }
                TokenType::NewLine | TokenType::DotComma => {
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

    pub fn statement(&mut self) -> Result<Statement, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            let res = match token.token_type {
                TokenType::Var => {
                    self.next_token();
                    let identifier = self.require_token(TokenType::Ident)?.token_info;
                    let value = self.define_var_initialization()?;
                    Statement::DefineVariable {
                        identifier,
                        value,
                        token_info: token.token_info,
                    }
                }
                TokenType::Const => {
                    self.next_token();
                    let identifier = self.require_token(TokenType::Ident)?.token_info;
                    let value = self.define_var_initialization()?;
                    Statement::DefineConstantVariable {
                        identifier,
                        value,
                        token_info: token.token_info,
                    }
                }
                _ => {
                    self.index = self.skip_new_lines();
                    let expr = self.expression_without_newline()?;
                    Statement::ExpressionStatement(expr)
                }
            };
            self.require_statement_end()?;
            return Ok(res)
        }

        Err(SyntaxError::ExpectedStatement{token_info: self.get_current_token_info()})
    }

    fn define_var_initialization(&mut self) -> Result<Option<(TokenInfo, Expression)>, SyntaxError> {
        let mut value = None;
        if let Some(&equal_token) = self.peek_token() {
            if equal_token.token_type == TokenType::Equal {
                self.next_token();
                let expr = self.expression_without_newline()?;
                value = Some((equal_token.token_info, expr));
            }
        }
        Ok(value)
    }
}