use std::{cell::RefCell, rc::Rc};

use crate::interpreter::{library::{exception::Exception, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct BlockStm{
    statements: RefCell<Vec<Box<dyn Statement>>>,
}

impl Statement for BlockStm {
    fn interpret(&self,context:Rc<Context>)-> Result<Task, Exception> {
        for stm in self.statements.borrow().iter(){
            let res = stm.interpret(context.clone());

            match res {
                Ok(o) => match o {
                    Task::Default => (),
                    _=> return Ok(o)
                },
                Err(e) => return Err(e),
            }
        }
        Ok(Task::Default)
    }
}

impl BlockStm {
    pub fn new() -> BlockStm{
        BlockStm{
            statements : RefCell::new(vec![]),
         }
    }
    pub fn add(&self , statement: Box<dyn Statement> ){
        self.statements.borrow_mut().push(statement);
    }
}