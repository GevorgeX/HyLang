use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::expression::Expression;
use crate::parser::Parser;

impl Parser{
    pub fn parse_call_arguments(&mut self) -> Result<(Vec<Expression>, TokenInfo), SyntaxError> {
        let mut arguments = Vec::new();
        while let Some(&next_token) = self.peek_token() {
            if next_token.token_type == TokenType::RightRBracket {
                break
            }
            else {
                let exp = self.expression()?;
                arguments.push(exp);
                if let Some(token) = self.peek_token() {
                    if token.token_type == TokenType::Comma {
                        self.next_token();
                        continue
                    } else if token.token_type == TokenType::RightRBracket {
                        break
                    } else {
                        return Err(SyntaxError::ExpectedToken {token_info: token.token_info, expected: TokenType::RightRBracket});
                    }
                }
                return Err(SyntaxError::ExpectedToken {token_info: next_token.token_info, expected: TokenType::RightRBracket});
            }
        }
        let right_rbracket_token = self.require_token(TokenType::RightRBracket)?;
        Ok((arguments, right_rbracket_token.token_info))
    }
}
