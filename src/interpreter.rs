mod expression;
mod statement;
mod library;
mod task;

use std::{cell::RefCell, rc::Rc};
use library::LocalContext;

use super::lexer::token::Token;
use self::statement::{block_stm::BlockStm, Statement};

pub type MemRef = Rc<LocalContext>;
pub struct Interpreter{
    index: RefCell<usize> ,
    pub memory: Rc<LocalContext>,
    tokens: Vec<Token>
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter{
        Interpreter{
            index: RefCell::new(0),
            memory: Rc::new(LocalContext::new()),
            tokens
        }
    }
    pub(self) fn get_token(&self)-> &Token {
        if *self.index.borrow() < self.tokens.len(){

            return &self.tokens[*self.index.borrow()]
        }
        
        &Token::EOF
    }
    pub(self) fn next_token(&self) {
        *self.index.borrow_mut() += 1;
    } 

    pub(self) fn require_token(&self , target:Token) {
        if *self.get_token() == target{
            self.next_token();
        }
        else{
            panic!("Require {}", target.to_string())
        }
    }
    pub fn parse_code(&self) -> Box<dyn Statement> {
        let  res = BlockStm::new(self.memory.clone()) ;

        loop {
            match self.get_token() {
                Token::EOF => {
                    // self.next_token();
                    break;
                },
                _ => {
                    res.add(self.statement());  
                }
            }
        };

        Box::new(res)
    }
}
