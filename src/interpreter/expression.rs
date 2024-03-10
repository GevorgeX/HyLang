mod number_exp;
pub(super) mod binary_exp;
mod unary_exp;
pub(super) mod value_exp;
mod boolean_exp;
mod conditional_exp;
mod char_exp;
mod array_exp;
mod functional_call_exp;

use std::rc::Rc;

use super::{library::{exception::Exception, object::ReferenceToObject, Context}, Interpreter};
use crate::lexer::token::Token;


use self::{
    array_exp::ArrayExpImpl, binary_exp::BinaryExpImpl, boolean_exp::BooleanExpImpl, char_exp::CharExpImpl, conditional_exp::ConditionalExpImpl, functional_call_exp::FunctionCallExpImpl, number_exp::NumberExpImpl, unary_exp::UnaryExpImpl, value_exp::ValueExpImpl
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
    Remain,
}
impl OperationType {
    pub fn to_string(&self) -> String{
        match self {
            OperationType::Plus => "+".to_string(),
            OperationType::Minus => "-".to_string(),
            OperationType::Multi => "*".to_string(),
            OperationType::Divide => "/".to_string(),
            OperationType::And => "&&".to_string(),
            OperationType::Or => "||".to_string(),
            OperationType::More => ">".to_string(),
            OperationType::Less =>"<+>".to_string(),
            OperationType::MoreOrEq =>">=".to_string(),
            OperationType::LessOrEq => "<=>".to_string(),
            OperationType::DoubleEqual => "==".to_string(),
            OperationType::NotEqual => "!=".to_string(),
            OperationType::Not => "!".to_string(),
            OperationType::Remain => "%".to_string(),
        }
    }
}
#[derive(Clone)]
pub enum  Expression{
    ArrayExp(ArrayExpImpl),
    BinaryExp(BinaryExpImpl),
    BooleanExp(BooleanExpImpl),
    CharExp(CharExpImpl),
    ConditionalExp(ConditionalExpImpl),
    FunctionCallExp(FunctionCallExpImpl),
    NumberExp(NumberExpImpl),
    UnaryExp(UnaryExpImpl),
    ValueExp(ValueExpImpl)
    }
impl Expression {
    pub fn evaluate(&self , context:Rc<Context>) -> Result<ReferenceToObject,Exception>{
        match self {
            Expression::ArrayExp(o) => o.evaluate(context),
            Expression::BinaryExp(o) => o.evaluate(context),
            Expression::BooleanExp(o) => o.evaluate(),
            Expression::CharExp(o) => o.evaluate(),
            Expression::ConditionalExp(o) => o.evaluate(context),
            Expression::FunctionCallExp(o) => o.evaluate(context),
            Expression::NumberExp(o) => o.evaluate(context),
            Expression::UnaryExp(o) => o.evaluate(context),
            Expression::ValueExp(o) => o.evaluate(context),
        }
    }
}

impl Interpreter {
    pub fn expression(&self)-> Result<Box<Expression>,Exception> {
        self.conditional()
    }
    fn conditional(&self) -> Result<Box<Expression>,Exception> {
        let mut res = match self.additive(){
            Ok(un) => un,
            Err(e) => return Err(e),
        };
        
        loop {
            let token = self.get_token();
            if  *token != Token::EOF{
                match token {
                    Token::And =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::And)));
                        }
                        else{
                            return add;
                        } 
                    },
                    Token::Or =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::Or)));
                        }
                        else{
                            return add;
                        } 
                    },
                    Token::DoubleEqual =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::DoubleEqual)));
                        }
                        else{
                            return add;
                        }                                            
                    }
                    Token::More =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::More)));
                        }
                        else{
                            return add;
                        }                         
                    },
                    Token::MoreEqual =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::MoreOrEq)));
                        }
                        else{
                            return add;
                        }                         
                    },
                    Token::Less =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::Less)));
                        }
                        else{
                            return add;
                        }                                            
                    },
                    Token::LessEqual =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::LessOrEq)));
                        }
                        else{
                            return add;
                        }                                            
                    },
                    Token::NotEqual =>{
                        self.next_token();
                        let add = self.additive();
                        if let Result::Ok(add) = add{
                            res = Box::new(Expression::ConditionalExp(ConditionalExpImpl::new(res, add, OperationType::NotEqual)));
                        }
                        else{
                            return add;
                        }                                            
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
    
        }
        return Ok(res)
    }
    fn additive(&self)-> Result<Box<Expression>,Exception> {
        let mut res = match self.multiplicative(){
            Ok(mult) => mult,
            Err(e) => return Err(e),
        };
        
        loop {
            let token = self.get_token();

            if  *token != Token::EOF{
                match token {
                    Token::Plus =>{
                        self.next_token();
                        let mult = self.multiplicative();
                        if let Result::Ok(mult) = mult{
                            res = Box::new(Expression::BinaryExp(BinaryExpImpl::new(res, mult, OperationType::Plus)));
                        }
                        else{
                            return mult;
                        } 
                    },
                    Token::Minus =>{
                        self.next_token();
                        let mult = self.multiplicative();
                        if let Result::Ok(mult) = mult{
                            res = Box::new(Expression::BinaryExp(BinaryExpImpl::new(res, mult, OperationType::Minus)));
                        }
                        else{
                            return mult;
                        } 
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
        }
    
        Ok(res)
    }
    fn multiplicative(&self ,)-> Result<Box<Expression>,Exception> {
        let mut res = match self.unary(){
            Ok(un) => un,
            Err(e) => return Err(e),
        };
    
        loop {
            let token = self.get_token();
            if  *token != Token::EOF{
                match token {
                    Token::Star =>{
                        self.next_token();
                        let un = self.unary();
                        if let Result::Ok(un) = un{
                            res = Box::new(Expression::BinaryExp(BinaryExpImpl::new(res, un, OperationType::Multi)));
                        }
                        else{
                            return un;
                        }
                    },
                    Token::Slash =>{
                        self.next_token();
                        let un = self.unary();
                        if let Result::Ok(un) = un{
                            res = Box::new(Expression::BinaryExp(BinaryExpImpl::new(res, un, OperationType::Divide)));
                        }
                        else{
                            return un;
                        }                    
                    },
                    Token::Percent =>{
                        self.next_token();
                        let un = self.unary();
                        if let Result::Ok(un) = un{
                            res = Box::new(Expression::BinaryExp(BinaryExpImpl::new(res, un, OperationType::Remain)));
                        }
                        else{
                            return un;
                        }                    
                    },
                    _ => break,
                }
            }
            else {
                break;
            }
    
        }
        return Ok(res)
    
    }
    fn unary(&self )-> Result <Box<Expression>,Exception> {
        
        let token = self.get_token();
        
        match token {
            Token::Plus => {
                self.next_token();
                self.primary()
            },
            Token::Minus => {
                self.next_token();
                let res = self.primary();
                if let Result::Ok(res) = res{
                     Ok(Box::new(Expression::UnaryExp(UnaryExpImpl::new(res, OperationType::Minus))))
                }
                else{
                    res
                }
            },
            Token::Not =>{
                self.next_token();
                let res = self.primary();
                if let Result::Ok(res) = res{
                    Ok(Box::new(Expression::UnaryExp(UnaryExpImpl::new(res, OperationType::Not))))
                }
                else{
                    res
                }          
            }
            _ => self.primary()
        }
    }
    fn primary(&self)->Result<Box<Expression>,Exception> { 
        let token = self.get_token();
        match token {
            Token::Number(num) => {
                self.next_token();
                Ok(Box::new(Expression::NumberExp(NumberExpImpl::new(*num))))
            },
            Token::TrueFalse(val) =>{
                self.next_token();
                Ok(Box::new(Expression::BooleanExp(BooleanExpImpl::new(*val ))))
            }
            Token::LeftParenthesis =>{
                self.next_token();
                let exp = self.expression();
                
                if let Err(e) = self.require_token(Token::RightParenthesis){
                    return Err(e);
                }
                return exp
            },
            Token::Apostr =>{
                self.next_token();
                if let Token::Word(word) = self.get_token() {

                    self.next_token();
                    if let Err(e) = self.require_token(Token::Apostr){
                        return Err(e);
                    }
                    Ok(Box::new(Expression::CharExp(CharExpImpl::new(word.clone()))))
                    
                }
                else{
                    return Err(Exception::require_symbol("բառ".to_string()));
                }
                
            },
            Token::LeftSquareBrace =>{
                self.next_token();
                let mut exps = vec![];
                while *self.get_token() != Token::RightSquareBrace {
                    let exp = self.expression();
                    if let Result::Ok(res) = exp {
                        exps.push(res)
                    }
                    else{
                        return exp;
                    }
                    
                    if *self.get_token() != Token::RightSquareBrace {
                        if let Err(e) = self.require_token(Token::Comma){
                            return Err(e);
                        }
                    }
                }
                
                self.next_token();
                Ok(Box::new(Expression::ArrayExp(ArrayExpImpl::new(exps))))
            },
            Token::Word(word)=>{
                self.next_token();
                if *self.get_token() == Token::LeftParenthesis{
                    self.next_token();
                    return self.function_call_with_return_value(word.clone())
                }
                Ok(Box::new(Expression::ValueExp(ValueExpImpl::new(word.clone()))))
            },
            _ => Err(Exception::unknow_word())
        }
    }
    fn function_call_with_return_value(&self, name:String) ->Result<Box<Expression>,Exception> {
        let mut args = vec![];   
        while Token::RightParenthesis !=  *self.get_token() {
                
            let value =match self.expression(){
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            args.push(value);
            // self.next_token();

            if *self.get_token() != Token::RightParenthesis {
                if let Err(e) = self.require_token(Token::Comma){
                    return Err(e);
                }
            }
        }
        self.next_token();

        Ok(Box::new(Expression::FunctionCallExp(FunctionCallExpImpl::new(name, args))))
    }
}
