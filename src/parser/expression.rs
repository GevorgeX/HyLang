mod if_else;
mod call_arguments;
pub mod block;
mod while_exp;

use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
use crate::parser::expression::block::BlockOfStatements;
pub use crate::parser::expression::if_else::{ElseBranch, IfElseBranch};
use crate::parser::Parser;

pub enum Expression {
    Integer{token_info: TokenInfo},
    Boolean{token_info: TokenInfo},
    Identifier{token_info: TokenInfo},
    Unary{operator: UnaryOperator, right: Box<Expression>, token_info: TokenInfo},
    Binary{left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>, token_info: TokenInfo},
    GetMember{object: Box<Expression>, member: Box<Expression>, token_info: TokenInfo},
    Call{callee: Box<Expression>, arguments: Vec<Expression>, token_info: (TokenInfo, TokenInfo)},
    Index{object: Box<Expression>, index: Box<Expression>, token_info: (TokenInfo, TokenInfo)},
    While{condition: Box<Expression>, body: BlockOfStatements, token_info: TokenInfo},
    Block{body: BlockOfStatements },
    IfElse{if_block: Box<IfElseBranch>, else_if_blocks: Option<Vec<IfElseBranch>>, else_block: Option<ElseBranch>},
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    LogicalNot,
    BitwiseNot,
    AddressOf,
    Dereference,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    AddAssign,
    Subtract,
    SubtractAssign,
    Multiply,
    MultiplyAssign,
    Divide,
    DivideAssign,
    Mod,
    ModAssign,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Assign,

    BitwiseAnd,
    BitwiseAndAssign,
    BitwiseOr,
    BitwiseOrAssign,
    BitwiseXor,
    BitwiseXorAssign,
    LeftShift,
    LeftShiftAssign,
    RightShift,
    RightShiftAssign,

}

impl Parser {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        self.expression_wrapper(true)
    }

    pub fn expression_without_newline(&mut self) -> Result<Expression, SyntaxError> {
        self.expression_wrapper(false)
    }

    fn expression_wrapper(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        self.assignment(ignore_newline)
    }

    fn assignment(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let left = self.logical_or(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Equal => BinaryOperator::Assign,
                TokenType::PlusEqual => BinaryOperator::AddAssign,
                TokenType::MinusEqual => BinaryOperator::SubtractAssign,
                TokenType::StarEqual => BinaryOperator::MultiplyAssign,
                TokenType::SlashEqual => BinaryOperator::DivideAssign,
                TokenType::PercentEqual => BinaryOperator::ModAssign,
                TokenType::AmpersandEqual => BinaryOperator::BitwiseAndAssign,
                TokenType::PipeEqual => BinaryOperator::BitwiseOrAssign,
                TokenType::CaretEqual => BinaryOperator::BitwiseXorAssign,
                TokenType::DoubleLessEqual => BinaryOperator::LeftShiftAssign,
                TokenType::DoubleGreaterEqual => BinaryOperator::RightShiftAssign,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.assignment(ignore_newline)?;
            return Ok(Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            });

        }

        Ok(left)
    }

    fn logical_or(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.logical_and(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Or => BinaryOperator::LogicalOr,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.logical_and(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn logical_and(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_or(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::And => BinaryOperator::LogicalAnd,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.bitwise_or(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_or(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_xor(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Pipe => BinaryOperator::BitwiseOr,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.bitwise_xor(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_xor(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_and(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Caret => BinaryOperator::BitwiseXor,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.bitwise_and(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_and(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.equivalence(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Ampersand => BinaryOperator::BitwiseAnd,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.equivalence(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn equivalence(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.relational(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::DoubleEqual => BinaryOperator::Equal,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.relational(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn relational(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.shift(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.shift(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn shift(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.additive(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::DoubleLess => BinaryOperator::LeftShift,
                TokenType::DoubleGreater => BinaryOperator::RightShift,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.additive(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn additive(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.multiplicative(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.multiplicative(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn multiplicative(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.prefix(ignore_newline)?;

        while let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                TokenType::Percent => BinaryOperator::Mod,
                _ => break,
            };
            self.next(ignore_newline);
            let right = self.prefix(ignore_newline)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn prefix(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        if let Some(&token) = self.peek(ignore_newline) {
            let operator = match token.token_type {
                TokenType::Minus => UnaryOperator::Negate,
                TokenType::Not => UnaryOperator::LogicalNot,
                TokenType::Tilde => UnaryOperator::BitwiseNot,
                TokenType::Ampersand => UnaryOperator::AddressOf,
                TokenType::Star => UnaryOperator::Dereference,
                _ => return self.postfix(ignore_newline),
            };
            self.next(ignore_newline);
            let right = self.prefix(ignore_newline)?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            });
        }
        self.postfix(ignore_newline)
    }

    fn postfix(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let mut left = self.primary(ignore_newline)?;
        while let Some(&token) = self.peek(ignore_newline) {
            match token.token_type {
                TokenType::Dot => {
                    self.next(ignore_newline);
                    let member = self.primary(ignore_newline)?;
                    left = Expression::GetMember {
                        object: Box::new(left),
                        member: Box::new(member),
                        token_info: token.token_info,
                    };
                }
                TokenType::LeftRBracket => {
                    self.next(ignore_newline);
                    let (arguments, right_rbracket_token_info) = self.parse_call_arguments()?;
                    left = Expression::Call {
                        callee: Box::new(left),
                        arguments,
                        token_info: (token.token_info, right_rbracket_token_info),
                    };
                }
                TokenType::LeftSBracket => {
                    self.next(ignore_newline);
                    let index = self.expression()?;
                    let right_sbracket_token = self.require_token(TokenType::RightSBracket)?;
                    left = Expression::Index {
                        object: Box::new(left),
                        index: Box::new(index),
                        token_info: (token.token_info, right_sbracket_token.token_info),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn primary(&mut self, ignore_newline: bool) -> Result<Expression, SyntaxError> {
        let token = {
            let token_info = self.get_current_token_info();
            self.next(ignore_newline).ok_or(SyntaxError::ExpectedExpression { token_info })
        }?.clone();

        match &token.token_type {
            TokenType::Integer => Ok(Expression::Integer { token_info: token.token_info }),
            TokenType::True | TokenType::False => Ok(Expression::Boolean { token_info: token.token_info }),
            TokenType::Ident => Ok(Expression::Identifier { token_info: token.token_info }),
            TokenType::LeftRBracket => {
                let expr = self.expression_wrapper(true)?;
                self.require_token(TokenType::RightRBracket)?;
                Ok(expr)
            }
            TokenType::While => {
                self.while_exp(token.token_info)
            }
            TokenType::LeftCBracket => {
                self.block()
            }
            TokenType::If => {
                self.if_else(token.token_info)
            }
            _ => Err(SyntaxError::UnexpectedToken { token: token.clone() }),
        }
    }
}