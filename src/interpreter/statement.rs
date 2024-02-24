use crate::lexer::token::Token;
use super::expression::{binary_exp, value_exp, OperationType};
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

// !!! TEMP
mod print_stm; use print_stm::PrintStm;

use block_stm::BlockStm;
use if_else_stm::IfElseStm;
use assignment_stm::AssignmentStm;
use define_variable_stm::DefineVariableStm;
use while_stm::WhileStm;
use break_stm::BreakStm;
use continue_stm::ContinueStm;

pub trait Statement {
    fn interpret(&self , context: &Context) -> Task;  
}


impl<'a> Interpreter<'a> {
    pub fn statement(&self) -> Box<dyn Statement> {
        match self.get_token() {
            Token::IF => {
                self.next_token();
                return self.if_else();
            },
            Token::Define =>{
                self.next_token();
                return self.define();
                
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
                    _ => panic!("Require = ")
                }
                 
 
            },
            Token::Break => {
                self.next_token();
                return Box::new(BreakStm::new())
            },
            Token::Continue => {
                self.next_token();
                return Box::new(ContinueStm::new())
            },
            Token::P_R_I_N_T =>{
                self.next_token();
                return  Box::new(PrintStm::new(self.expression()));
            }
            _ => panic!(""),
        }
        
    }
    fn while_stm(&self) -> Box<dyn Statement>{
        let condition = self.expression();
        let while_statements = self.block();

        Box::new(WhileStm::new(condition, while_statements))
    }
    fn block(&self) -> Box<dyn Statement> {
        let res = BlockStm::new( );
        self.require_token(Token::LeftBrace);

        while *self.get_token() != Token::RightBrace {
            res.add(self.statement());
        }
        self.next_token();
        Box::new(res)

    }
    fn if_else(&self ) -> Box<dyn Statement>{
        let condition = self.expression();
        let if_statement = self.block();
        let else_statemnt;

        if *self.get_token() == Token::ELSE{
            self.next_token();
            else_statemnt = Some(self.block())
        }
        else{
            else_statemnt = None;
        }

        Box::new(IfElseStm::new(condition, if_statement ,else_statemnt))
    }
    fn define(&self) ->Box<dyn Statement>{
        if let Token::Word(word) = self.get_token(){
            self.next_token();
            self.require_token(Token::Equal);
            return Box::new(DefineVariableStm::new(word.clone(), self.expression() , ))
        }
        else{
            panic!("Require name");
        }
    }

    fn assignment(&self,word:&String) -> Box<dyn Statement> {
        return Box::new(AssignmentStm::new(word.clone(), self.expression() , ))
    }
    fn assignment_with_modifare(&self,word:&String, op:OperationType) -> Box<dyn Statement> {
        let val = Box::new(binary_exp::BinaryExp::new(
            Box::new(value_exp::ValueExp::new(word.clone())),
            self.expression(),
            op
         ));

        return Box::new(AssignmentStm::new(word.clone(), val , ))
    }
}
