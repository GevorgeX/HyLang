mod number_exp;
pub(super) mod binary_exp;
mod unary_exp;
pub(super) mod value_exp;
mod boolean_exp;
mod conditional_exp;
mod char_exp;
mod array_exp;

use std::rc::Rc;

use crate::interpreter::library::object::ReferenceToObject;
use super::{library::{exception::Exception, Context}, Interpreter};
use crate::lexer::token::Token;


use self::{
    binary_exp::BinaryExp, boolean_exp::BooleanExp, char_exp::CharExp, conditional_exp::ConditionalExp, number_exp::NumberExp, unary_exp::UnaryExp, value_exp::ValueExp,array_exp::ArrayExp
};
#[derive(Clone)]
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

dyn_clone::clone_trait_object!(Expression);
pub trait Expression : dyn_clone::DynClone{
    fn evaluate(&self , context:Rc<Context>) -> Result<ReferenceToObject,Exception>;
}



impl Interpreter {
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
    fn unary(&self )-> Result <Box<dyn Expression>,Exception> {
        
        let token = self.get_token();
        
        match token {
            Token::Plus => {
                self.next_token();
                return self.primary()
            },
            Token::Minus => {
                self.next_token();
                let res = self.primary();
                if let Result::Ok(res) = res{
                    return Ok(Box::new(UnaryExp::new(res, OperationType::Minus)))
                }
                else{
                    return res;
                }
            },
            Token::Not =>{
                self.next_token();
                let res = self.primary();
                if let Result::Ok(res) = res{
                    return Ok(Box::new(UnaryExp::new(res, OperationType::Not)))
                }
                else{
                    return res;
                }            }
            _ => self.primary()
        }
    }
    fn primary(&self)->Result<Box<dyn Expression>,Exception> { 
        let token = self.get_token();
        match token {
            Token::Number(num) => {
                self.next_token();
                return Ok(Box::new(NumberExp::new(*num)))
            },
            Token::TrueFalse(val) =>{
                self.next_token();
                return  Ok(Box::new(BooleanExp::new(*val )));
            }
            Token::LeftParenthesis =>{
                self.next_token();
                let exp = self.expression();
                
                if let Err(e) = self.require_token(Token::RightParenthesis){
                    return Err(e);
                }
                return Ok(exp)
            },
            Token::Apostr =>{
                self.next_token();
                if let Token::Word(word) = self.get_token() {

                    self.next_token();
                    if let Err(e) = self.require_token(Token::Apostr){
                        return Err(e);
                    }
                    return Ok(Box::new(CharExp::new(word.clone())))
                    
                }
                else{
                    return Err(Exception::new_require_symbol("բառ".to_string()));
                }
                
            },
            Token::LeftSquareBrace =>{
                self.next_token();
                let mut exps = vec![];
                while *self.get_token() != Token::RightSquareBrace {
                    exps.push(self.expression());
                    
                    if *self.get_token() != Token::RightSquareBrace {
                        if let Err(e) = self.require_token(Token::Comma){
                            return Err(e);
                        }
                    }
                }
                
                self.next_token();
                
                return Ok(Box::new(ArrayExp::new(exps)));
            },
            Token::Word(word)=>{
                self.next_token();
                return Ok(Box::new(ValueExp::new(word.clone())));
            },
            _ => Err(Exception::new_unknow_word())
        }
    }
}
