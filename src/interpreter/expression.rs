mod number_exp;
pub(super) mod binary_exp;
mod unary_exp;
pub(super) mod value_exp;
mod boolean_exp;
mod conditional_exp;
mod char_exp;
mod array_exp;

use crate::interpreter::library::ReferenceToObject;
use super::{library::Context, Interpreter};
use crate::lexer::token::Token;

use self::{
    binary_exp::BinaryExp, boolean_exp::BooleanExp, char_exp::CharExp, conditional_exp::ConditionalExp, number_exp::NumberExp, unary_exp::UnaryExp, value_exp::ValueExp
};
pub enum OperationType {
    Plus,
    Minus,
    Multi,
    Divide,
    And,
    Or,
    More,
    Less,
    MoreOrEq,
    LessOrEq,
    DoubleEqual,
    NotEqual,
    Not,
}

pub trait Expression{
    fn evaluate(&self , context:&Context) -> ReferenceToObject;
}



impl<'a> Interpreter<'a> {
    pub fn expression(&self)-> Box<dyn Expression> {
        self.conditional()
    }
    fn conditional(&self) -> Box<dyn Expression> {
        let mut res = self.additive();
    
        loop {
            let token = self.get_token();
            if  *token != Token::EOF{
                match token {
                    Token::And =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::And));
                    },
                    Token::Or =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::Or));
                    },
                    Token::DoubleEqual =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::DoubleEqual));
                                           
                    }
                    Token::More =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::More));
                        
                    },
                    Token::MoreEqual =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::MoreOrEq));
                        
                    },
                    Token::Less =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::Less));
                                           
                    },
                    Token::LessEqual =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::LessOrEq));
                                           
                    },
                    Token::NotEqual =>{
                        self.next_token();
                        res = Box::new(ConditionalExp::new(res, self.additive(), OperationType::NotEqual));
                                           
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
    
        }
        return res
    }
    fn additive(&self)-> Box<dyn Expression> {
        let mut res = self.multiplicative();
    
        loop {
            let token = self.get_token();

            if  *token != Token::EOF{
                match token {
                    Token::Plus =>{
                        self.next_token();
                        res = Box::new(BinaryExp::new(res, self.multiplicative(), OperationType::Plus));
                    },
                    Token::Minus =>{
                        self.next_token();
                        res = Box::new(BinaryExp::new(res, self.multiplicative(), OperationType::Minus));
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
        }
    
        res
    }
    fn multiplicative(&self ,)-> Box<dyn Expression> {
        let mut res = self.unary();
    
        loop {
            let token = self.get_token();
            if  *token != Token::EOF{
                match token {
                    Token::Star =>{
                        self.next_token();
                        res = Box::new(BinaryExp::new(res, self.unary(), OperationType::Multi));
                    },
                    Token::Slash =>{
                        self.next_token();
                        res = Box::new(BinaryExp::new(res, self.unary(), OperationType::Divide));
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
    
        }
        return res
    
    }
    fn unary(&self )-> Box<dyn Expression> {
        
        let token = self.get_token();
        
        match token {
            Token::Plus => {
                self.next_token();
                return self.primary()
            },
            Token::Minus => {
                self.next_token();
                return Box::new(UnaryExp::new(self.primary(), OperationType::Minus))
            },
            Token::Not =>{
                self.next_token();
                return Box::new(UnaryExp::new(self.primary(), OperationType::Not))
            }
            _ => self.primary()
        }
    }
    fn primary(&self)-> Box<dyn Expression> { 
        let token = self.get_token();
        match token {
            Token::Number(num) => {
                self.next_token();
                return Box::new(NumberExp::new(*num))
            },
            Token::TrueFalse(val) =>{
                self.next_token();
                return  Box::new(BooleanExp::new(*val ));
            }
            Token::LeftParenthesis =>{
                self.next_token();
                let exp = self.expression();
                
                self.require_token(Token::RightParenthesis);
                return exp
            },
            Token::Apostr =>{
                self.next_token();
                if let Token::Word(word) = self.get_token() {

                    self.next_token();
                    self.require_token(Token::Apostr);
                    return Box::new(CharExp::new(word.clone()))
                    
                }
                else{
                    panic!("Require symbol")
                }
                
            },
            Token::Word(word)=>{
                self.next_token();
                return Box::new(ValueExp::new(word.clone()));
            },
            _ => panic!("Symbol error {}", self.index.borrow())
        }
    }
}
