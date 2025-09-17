mod control_flow;

use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
pub(crate) use crate::parser::expression::control_flow::{ElseBranch, IfElseBranch};
use crate::parser::statement::Statement;
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
    While{condition: Box<Expression>, body: Vec<Statement>, token_info: TokenInfo, brackets_token_info: (TokenInfo, TokenInfo)},
    Block{body: Vec<Statement>, brackets_token_info: (TokenInfo, TokenInfo)},
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, SyntaxError> {
        let left = self.logical_or()?;

        while let Some(&token) = self.peek_token() {
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
            self.next_token();
            let right = self.assignment()?;
            return Ok(Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            });

        }

        Ok(left)
    }

    fn logical_or(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.logical_and()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Or => BinaryOperator::LogicalOr,
                _ => break,
            };
            self.next_token();
            let right = self.logical_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_or()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::And => BinaryOperator::LogicalAnd,
                _ => break,
            };
            self.next_token();
            let right = self.bitwise_or()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_or(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_xor()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Pipe => BinaryOperator::BitwiseOr,
                _ => break,
            };
            self.next_token();
            let right = self.bitwise_xor()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_xor(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.bitwise_and()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Caret => BinaryOperator::BitwiseXor,
                _ => break,
            };
            self.next_token();
            let right = self.bitwise_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn bitwise_and(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.equivalence()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Ampersand => BinaryOperator::BitwiseAnd,
                _ => break,
            };
            self.next_token();
            let right = self.equivalence()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn equivalence(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.relational()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::DoubleEqual => BinaryOperator::Equal,
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
        let mut left = self.shift()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::DoubleEqual => BinaryOperator::Equal,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };
            self.next_token();
            let right = self.shift()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn shift(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.additive()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::DoubleLess => BinaryOperator::LeftShift,
                TokenType::DoubleGreater => BinaryOperator::RightShift,
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
        let mut left = self.prefix()?;

        while let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                TokenType::Percent => BinaryOperator::Mod,
                _ => break,
            };
            self.next_token();
            let right = self.prefix()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            };
        }

        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expression, SyntaxError> {
        if let Some(&token) = self.peek_token() {
            let operator = match token.token_type {
                TokenType::Minus => UnaryOperator::Negate,
                TokenType::Not => UnaryOperator::LogicalNot,
                TokenType::Tilde => UnaryOperator::BitwiseNot,
                TokenType::Ampersand => UnaryOperator::AddressOf,
                TokenType::Star => UnaryOperator::Dereference,
                _ => return self.postfix(),
            };
            self.next_token();
            let right = self.prefix()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
                token_info: token.token_info,
            });
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.primary()?;
        while let Some(&token) = self.peek_token() {
            match token.token_type {
                TokenType::Dot => {
                    self.next_token();
                    let member = self.primary()?;
                    left = Expression::GetMember {
                        object: Box::new(left),
                        member: Box::new(member),
                        token_info: token.token_info,
                    };
                }
                TokenType::LeftRBracket => {
                    self.next_token();
                    let (arguments, right_rbracket_token_info) = self.parse_call_arguments()?;
                    left = Expression::Call {
                        callee: Box::new(left),
                        arguments,
                        token_info: (token.token_info, right_rbracket_token_info),
                    };
                }
                TokenType::LeftSBracket => {
                    self.next_token();
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

    fn parse_call_arguments(&mut self) -> Result<(Vec<Expression>, TokenInfo), SyntaxError> {
        let mut arguments = Vec::new();
        if let Some(&next_token) = self.peek_token() {
            if next_token.token_type != TokenType::RightRBracket {
                loop {
                    let arg = self.expression()?;
                    arguments.push(arg);
                    if let Some(&comma_or_rbracket) = self.peek_token() {
                        if comma_or_rbracket.token_type == TokenType::Comma {
                            self.next_token();
                            continue;
                        } else if comma_or_rbracket.token_type == TokenType::RightRBracket {
                            break;
                        } else {
                            return Err(SyntaxError::ExpectedToken { token_info: comma_or_rbracket.token_info, expected: TokenType::Comma });
                        }
                    } else {
                        return Err(SyntaxError::ExpectedToken { token_info: self.get_current_token_info(), expected: TokenType::RightRBracket });
                    }
                }
            }
        }
        let right_rbracket_token = self.require_token(TokenType::RightRBracket)?;
        Ok((arguments, right_rbracket_token.token_info))
    }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        let token = {
            let token_info = self.get_current_token_info();
            self.next_token().ok_or(SyntaxError::ExpectedExpression { token_info })
        }?.clone();

        match &token.token_type {
            TokenType::Integer => Ok(Expression::Integer { token_info: token.token_info }),
            TokenType::True | TokenType::False => Ok(Expression::Boolean { token_info: token.token_info }),
            TokenType::Ident => Ok(Expression::Identifier { token_info: token.token_info }),
            TokenType::LeftRBracket => {
                let expr = self.expression()?;
                self.require_token(TokenType::RightRBracket)?;
                Ok(expr)
            }
            TokenType::While => {
                self.while_exp(token.token_info)
            }
            TokenType::LeftCBracket => {
                self.block(token.token_info)
            }
            TokenType::If => {
                self.if_else(token.token_info)
            }
            _ => Err(SyntaxError::UnexpectedToken { token: token.clone() }),
        }
    }
}