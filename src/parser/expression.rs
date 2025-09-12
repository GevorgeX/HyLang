use crate::errors::syntax_errors::SyntaxError;
use crate::lexer::token::{TokenInfo, TokenType};
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

pub struct IfElseBranch {
    pub(crate) condition: Expression,
    pub(crate) body: Vec<Statement>,
    pub(crate) token_info: TokenInfo,
    pub(crate) brackets_token_info: (TokenInfo, TokenInfo)
}

pub struct ElseBranch {
    pub(crate) body: Vec<Statement>,
    pub(crate) token_info: TokenInfo,
    pub(crate) brackets_token_info: (TokenInfo, TokenInfo)
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
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
    Greater
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
                TokenType::Not => UnaryOperator::Not,
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
        let left = self.primary();
        if let Some(&token) = self.peek_token() {
            match token.token_type {
                TokenType::Dot => {
                    self.next_token();
                    let member = self.primary()?;
                    let left = left?;
                    Ok(Expression::GetMember {
                        object: Box::new(left),
                        member: Box::new(member),
                        token_info: token.token_info,
                    })
                }
                TokenType::LeftRBracket => {
                    self.next_token();
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
                    let rbracket_token = self.require_token(TokenType::RightRBracket)?;
                    let left = left?;
                    Ok(Expression::Call {
                        callee: Box::new(left),
                        arguments,
                        token_info: (token.token_info, rbracket_token.token_info),
                    })
                }
                TokenType::LeftSBracket => {
                    self.next_token();
                    let index = self.expression()?;
                    let rbracket_token = self.require_token(TokenType::RightSBracket)?;
                    let left = left?;
                    Ok(Expression::Index {
                        object: Box::new(left),
                        index: Box::new(index),
                        token_info: (token.token_info, rbracket_token.token_info),
                    })
                }
                _ => left,
            }
        }
        else {
            left
        }
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

    fn while_exp(&mut self, token_info: TokenInfo) -> Result<Expression, SyntaxError> {
        let exp = self.expression()?;
        let left_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
        let (body, right_bracket) = self.block_of_statements()?;
        Ok(Expression::While {
            condition: Box::new(exp),
            body,
            token_info,
            brackets_token_info: (left_bracket, right_bracket),
        })
    }

    fn block(&mut self, left_bracket: TokenInfo) -> Result<Expression, SyntaxError> {
        let (body, right_bracket) = self.block_of_statements()?;
        Ok(Expression::Block {
            body,
            brackets_token_info: (left_bracket, right_bracket),
        })
    }

    fn block_of_statements(&mut self) -> Result<(Vec<Statement>,TokenInfo), SyntaxError> {
        let mut body = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::RightCBracket {
                break;
            }
            let stmt = self.statement()?;
            body.push(stmt);
        }
        let right_bracket = self.require_token(TokenType::RightCBracket)?;
        Ok((body, right_bracket.token_info))
    }

    fn if_else(&mut self, if_token: TokenInfo) -> Result<Expression, SyntaxError> {
        let condition = self.expression()?;
        let left_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
        let (if_body, right_bracket) = self.block_of_statements()?;
        let if_block = IfElseBranch {
            condition,
            body: if_body,
            token_info: if_token,
            brackets_token_info: (left_bracket, right_bracket),
        };

        let mut else_if_blocks = Vec::new();
        while let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::Elif {
                self.next_token();
                let elif_token = token.token_info;
                let condition = self.expression()?;
                let left_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
                let (elif_body, right_bracket) = self.block_of_statements()?;
                else_if_blocks.push(IfElseBranch {
                    condition,
                    body: elif_body,
                    token_info: elif_token,
                    brackets_token_info: (left_bracket, right_bracket),
                });
            } else {
                break;
            }
        }

        let else_block = if let Some(&token) = self.peek_token() {
            if token.token_type == TokenType::Else {
                self.next_token();
                let else_token = token.token_info;
                let left_bracket = self.require_token(TokenType::LeftCBracket)?.token_info;
                let (else_body, right_bracket) = self.block_of_statements()?;
                Some(ElseBranch {
                    body: else_body,
                    token_info: else_token,
                    brackets_token_info: (left_bracket, right_bracket),
                })
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