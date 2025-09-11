use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{ TokenInfo, TokenType};
use crate::parser::Parser;

pub enum Expression {
    Integer{token_info: TokenInfo},
    Boolean{token_info: TokenInfo},
    Identifier{token_info: TokenInfo},
    Unary{operator: UnaryOperator, right: Box<Expression>, token_info: TokenInfo},
    Binary{left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>, token_info: TokenInfo},
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    And,
    Or,
    Equal,
    NotEqual,
    Less,
    Greater,

}

impl Parser {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.relational()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Equal => BinaryOperator::Equal,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };
            self.next_token();
            let right = self.relational()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn relational(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Less => BinaryOperator::Less,
                TokenType::Greater => BinaryOperator::Greater,
                _ => break,
            };
            self.next_token();
            let right = self.bitwise()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.additive()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Pipe => BinaryOperator::Or,
                TokenType::Ampersand => BinaryOperator::And,
                _ => break,
            };
            self.next_token();
            let right = self.additive()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn additive(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.multiplicative()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.next_token();
            let right = self.multiplicative()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.postfix()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                TokenType::Percent => BinaryOperator::Mod,
                _ => break,
            };
            self.next_token();
            let right = self.postfix()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn postfix(&mut self) -> Result<Expression, SyntaxError> {
        self.prefix()
    }

    fn prefix(&mut self) -> Result<Expression, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Minus => UnaryOperator::Negate,
                TokenType::Not => UnaryOperator::Not,
                _ => return self.primary(),
            };
            self.next_token();
            let right = self.prefix()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            });
        }
        Err(SyntaxError::ExpectedExpression { token_info: self.get_current_token_info() })
    }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        let token = {
            let token_info = self.get_current_token_info();
            self.next_token().ok_or(SyntaxError::ExpectedExpression { token_info })
        }?;

        match &token.token_type {
            TokenType::Integer => Ok(Expression::Integer { token_info: token.token_info }),
            TokenType::True | TokenType::False => Ok(Expression::Boolean { token_info: token.token_info }),
            TokenType::Ident => Ok(Expression::Identifier { token_info: token.token_info }),
            TokenType::LeftRBracket => {
                let expr = self.expression()?;
                self.require_token(TokenType::RightRBracket)?;
                Ok(expr)
            }
            _ => Err(SyntaxError::UnexpectedToken { token: token.clone() }),
        }
    }
}