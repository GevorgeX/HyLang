mod expression;
mod statement;
mod library;
mod task;

use std::{cell::RefCell, rc::Rc};


use super::lexer::token::Token;
use self::{library::{exception::Exception, Context}, statement::{block_stm::BlockStm, Statement}};

pub struct Interpreter{
    index: RefCell<usize>,
    tokens: Vec<Token>,
    main_context:Rc<Context>,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter{
        Interpreter{
            index: RefCell::new(0),
            tokens,
            main_context: Context::new_module_context(None),
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

    pub(self) fn require_token(&self , target:Token)-> Result<(), Exception> {
        if *self.get_token() == target{
            self.next_token();
            return Ok(())
        }
        else{
            return Err(Exception::new_require_symbol(target.to_string()));
        }
    }
    pub fn parse_code(&self) -> Box<dyn Statement> {

        let res = BlockStm::new() ;
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
    pub fn run(&self, main_func_name:&String){
        self.parse_code().interpret(self.main_context.clone());
        match &*self.main_context.get_object(main_func_name) {
            library::object::Object::FunctionObject(func) => func.call(vec![], self.main_context.clone()),
            _=> ()
        }
    }
}
