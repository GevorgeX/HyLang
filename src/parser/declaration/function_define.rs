use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::types::Type;
use crate::parser::Parser;

pub struct FunctionArgument {
    pub name_token_info: TokenInfo,
    pub arg_type: Type,
}

impl Parser{
    pub fn function_argument(&mut self) -> Result<(TokenInfo,Vec<FunctionArgument>, TokenInfo), SyntaxError> {
        let lbracket_token = self.require_token(TokenType::LeftRBracket)?.token_info;
        let mut args = Vec::new();

        while let Some(&next_token) = self.peek_token() {
            if next_token.token_type == TokenType::RightRBracket {
                break
            }
            else {
                let name_token = self.require_token(TokenType::Ident)?.clone();
                let arg_type = self.parse_type()?;
                args.push(FunctionArgument {
                    name_token_info: name_token.token_info,
                    arg_type,
                });
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

        Ok((lbracket_token,args,self.require_token(TokenType::RightRBracket)?.token_info))
    }
}
