use std::rc::Rc;

use crate::lexer::token::Token;

use self::return_stm::ReturnStmImpl;

use super::expression::{binary_exp, value_exp, Expression, OperationType};
use super::library::exception::Exception;
use super::library::Context;
use super::task::Task;
use super::Interpreter;

pub mod block_stm;
mod assignment_stm;
mod define_variable_stm;
mod if_else_stm;
mod while_stm;
mod break_stm;
mod continue_stm;
mod define_function_stm;
mod function_call_stm;
mod return_stm;

// !!! TEMP
mod print_stm; use print_stm::PrintStmImpl;

use block_stm::BlockStmImpl;
use if_else_stm::IfElseStmImpl;
use assignment_stm::AssignmentStmImpl;
use define_variable_stm::DefineVariableStmImpl;
use while_stm::WhileStmImpl;
use break_stm::BreakStmImpl;
use continue_stm::ContinueStmImpl;
use define_function_stm::DefineFunctionStmImpl;
use function_call_stm::FunctionCallStmImpl;

#[derive(Clone)]
pub enum Statement {
    AssignmentStm(AssignmentStmImpl),
    BlockStm(BlockStmImpl),
    BreakStm(BreakStmImpl),
    ContinueStm(ContinueStmImpl),
    DefineFunctionStm(DefineFunctionStmImpl),
    DefineVariableStm(DefineVariableStmImpl),
    FunctionCallStm(FunctionCallStmImpl),
    IfElseStm(IfElseStmImpl),
    PRINTSTM(PrintStmImpl),
    ReturnStm(ReturnStmImpl),
    WhileStm(WhileStmImpl)
}  

impl Statement {   
    pub fn interpret(&self , context: Rc<Context>) -> Result<Task, Exception>{
        match self {
            Statement::AssignmentStm(o) => o.interpret(context),
            Statement::BlockStm(o) => o.interpret(context),
            Statement::BreakStm(o) => o.interpret(),
            Statement::ContinueStm(o) => o.interpret(),
            Statement::DefineFunctionStm(o) => o.interpret(context),
            Statement::DefineVariableStm(o) => o.interpret(context),
            Statement::FunctionCallStm(o) => o.interpret(context),
            Statement::IfElseStm(o) => o.interpret(context),
            Statement::PRINTSTM(o) => o.interpret(context),
            Statement::ReturnStm(o) => o.interpret(context),
            Statement::WhileStm(o) => o.interpret(context),
        }
    }
}

impl Interpreter {
    pub fn statement(&self) -> Result<Box<Statement>,Exception>{
        match self.get_token() {
            Token::IF => {
                self.next_token();
                return self.if_else();
            },
            Token::Define =>{
                self.next_token();
                return self.define_variable();
                
            },
            Token::LeftBrace =>{
                return self.block();
            }
            Token::While =>{
                self.next_token();
                return  self.while_stm();
            }
            Token::Word(word)=>{
                self.next_token();
                // self.require_token(Token::Equal);
                match self.get_token() {
                    Token::Equal =>{
                        self.next_token();
                        self.assignment(word)
                    },
                    Token::PlusEqual =>{
                        self.next_token();
                        self.assignment_with_modifare(word, OperationType::Plus)  
                    },
                    Token::MinusEqual =>{
                        self.next_token();
                        self.assignment_with_modifare(word, OperationType::Minus)  
                    },
                    Token::StarEqual =>{
                        self.next_token();
                        self.assignment_with_modifare(word, OperationType::Multi)
                    },
                    Token::SlashEqual =>{
                        self.next_token();
                        self.assignment_with_modifare(word, OperationType::Divide)  
                    },
                    Token::LeftParenthesis =>{
                        self.next_token();
                        self.function_call(word.clone())  
                    }
                    _ => Err(Exception::require_symbol("=".to_string()))
                }
                 
            },
            Token::Break => {
                self.next_token();
                Ok(Box::new(Statement::BreakStm(BreakStmImpl::new())))
            },
            Token::Continue => {
                self.next_token();
                Ok(Box::new(Statement::ContinueStm(ContinueStmImpl::new())))
            },
            Token::Return => {
                self.next_token();
                let exp = match self.expression() {
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };
                Ok(Box::new(Statement::ReturnStm(ReturnStmImpl::new(exp))))
            }
            Token::P_R_I_N_T =>{
                self.next_token();
                let exp = match self.expression() {
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };
                Ok(Box::new(Statement::PRINTSTM(PrintStmImpl::new(exp))))
            },
            Token::Function =>{
                self.next_token();
                return self.define_func()
            },
            _ => return Err(Exception::unknow_word()),
        }
        
    }
    fn while_stm(&self) -> Result<Box<Statement>,Exception>{
        let condition = match self.expression(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let while_statements = match self.block(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        Ok(Box::new(Statement::WhileStm(WhileStmImpl::new(condition, while_statements))))
    }
    fn block(&self) -> Result<Box<Statement>,Exception>{
        let mut res = BlockStmImpl::new( );
        if let Err(e) = self.require_token(Token::LeftBrace){
            return Err(e);
        }

        while *self.get_token() != Token::RightBrace {
            let statement = match self.statement(){
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            res.add(statement);
        }
        self.next_token();
        Ok(Box::new(Statement::BlockStm(res)))

    }
    fn if_else(&self ) -> Result<Box< Statement>,Exception>{
        let condition = match self.expression(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let if_statement = match self.block(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let else_statemnt;

        if *self.get_token() == Token::ELSE{
            self.next_token();
            let block =match self.block(){
                Ok(o) => o,
                Err(e) =>return  Err(e),
            };
            else_statemnt = Some(block)
        }
        else{
            else_statemnt = None;
        }

        Ok(Box::new(Statement::IfElseStm(IfElseStmImpl::new(condition, if_statement ,else_statemnt))))
    }
    fn define_variable(&self) ->Result<Box< Statement>,Exception>{
        if let Token::Word(word) = self.get_token(){
            self.next_token();
            if let Err(e) = self.require_token(Token::Equal){
                return Err(e);
            }

            let value =match self.expression(){
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            Ok(Box::new(Statement::DefineVariableStm(DefineVariableStmImpl::new(word.clone(), value))))
        }
        else{
            Err(Exception::require_symbol("name".to_string()))
        }
    }

    fn assignment(&self,word:&String) -> Result<Box<Statement>,Exception> {
        let value =match self.expression(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        
        Ok(Box::new(Statement::AssignmentStm(AssignmentStmImpl::new(word.clone(), value ))))
    }
    fn assignment_with_modifare(&self,word:&String, op:OperationType) -> Result<Box< Statement>,Exception> {
        let right =match self.expression(){
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        
        let val = Box::new(Expression::BinaryExp(binary_exp::BinaryExpImpl::new(
            Box::new(Expression::ValueExp(value_exp::ValueExpImpl::new(word.clone()))),
            right,
            op
         )));

        Ok(Box::new(Statement::AssignmentStm(AssignmentStmImpl::new(word.clone(), val)) ))
    }
    fn define_func(&self) ->Result<Box< Statement>,Exception> {
        if let Token::Word(word) = self.get_token(){
            self.next_token();
            if let Err(e) = self.require_token(Token::LeftParenthesis){
                return Err(e);
            }

            let mut args :Vec<String>= vec![];

            while  let Token::Word(word) = self.get_token() {
                
                args.push(word.clone());
                self.next_token();

                if *self.get_token() != Token::RightParenthesis {
                    if let Err(e) = self.require_token(Token::Comma){
                        return Err(e);
                    }                
                }
            }
            self.next_token();

            let body = match self.block(){
                Ok(o) => o,
                Err(e) => return Err(e),
            };


            Ok(Box::new(Statement::DefineFunctionStm(DefineFunctionStmImpl::new(word.clone(),args, body))))
            // self.statement()
        }
        else{
            Err(Exception::require_symbol("անուն".to_string()))
        }
    }
    fn function_call(&self, name:String) ->Result<Box<Statement>,Exception> {
        let mut args = vec![];      
        while  Token::RightParenthesis !=  *self.get_token() {
                
            let value = match self.expression(){
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

        Ok(Box::new(Statement::FunctionCallStm(FunctionCallStmImpl::new(name, args))))
    }
}

