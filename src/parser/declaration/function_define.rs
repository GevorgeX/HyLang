use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::Parser;

#[derive(Debug)]
pub struct FunctionArgument {
    name_token_info: TokenInfo,
    type_token_info: TokenInfo,
}

impl Parser{
    pub fn function_argument(&mut self) -> Result<(Vec<FunctionArgument>, TokenInfo), SyntaxError> {
        let mut args = Vec::new();

        while let Some(&token) = self.peek_token() {
            match token.token_type {
                TokenType::Ident => {
                    let name_token_info = self.require_token(TokenType::Ident)?.token_info;
                    let type_token_info = self.require_token(TokenType::Ident)? .token_info;
                    args.push(FunctionArgument { name_token_info, type_token_info });

                    if let Some(&next_token) = self.peek_token() {
                        if next_token.token_type == TokenType::Comma {
                            self.next_token();
                        } else if next_token.token_type == TokenType::RightRBracket {
                            break;
                        } else {
                            return Err(SyntaxError::ExpectedToken { token_info: next_token.token_info, expected: TokenType::RightRBracket });
                        }
                    } else {
                        return Err(SyntaxError::ExpectedToken { token_info: token.token_info, expected: TokenType::RightRBracket });
                    }
                }
                TokenType::RightRBracket => break, // end of arguments
                _ => return Err(SyntaxError::UnexpectedToken { token: token.clone() }),
            }
        }

        Ok((args,self.require_token(TokenType::RightRBracket)?.token_info))
    }
}
