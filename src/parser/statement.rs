use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::expression::Expression;
use crate::parser::Parser;

use super::types::Type;

mod variable_decl;

pub enum Statement {
    ExpressionStatement(Expression),
    DefineVariable{identifier: TokenInfo, value: Option<(TokenInfo, Expression)>, var_type: Option<Type>},
    DefineConstantVariable{identifier: TokenInfo, value: Option<(TokenInfo, Expression)>, const_type: Option<Type>},
}

impl Parser {
    fn require_statement_end(&mut self) -> Result<(), SyntaxError> {
        let mut found = false;
        while let Some(token) = self.peek_token_newline() {
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

    fn is_variable_decl_end(&self) -> bool {
        if let Some(token) = self.peek_token_newline() {
            return match token.token_type {
                TokenType::RightCBracket | TokenType::NewLine | TokenType::DotComma | TokenType::Equal => true,
                _ => false,
            }
        }
        false
    }

    pub fn statement(&mut self) -> Result<Statement, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            let res = match token.token_type {
                TokenType::Var => {
                    self.next_token();
                    let identifier = self.require_token(TokenType::Ident)?.token_info;
                    let var_type = self.define_var_type()?;
                    let value = self.var_initialization()?;

                    Statement::DefineVariable {
                        identifier,
                        value,
                        var_type
                    }
                }
                TokenType::Const => {
                    self.next_token();
                    let identifier = self.require_token(TokenType::Ident)?.token_info;
                    let const_type = self.define_var_type()?;
                    let value = self.var_initialization()?;

                    Statement::DefineConstantVariable {
                        identifier,
                        value,
                        const_type
                    }
                }
                _ => {
                    let expr = self.expression_newline()?;
                    Statement::ExpressionStatement(expr)
                }
            };
            self.require_statement_end()?;
            return Ok(res)
        }

        Err(SyntaxError::ExpectedStatement{token_info: self.get_current_token_info()})
    }

}