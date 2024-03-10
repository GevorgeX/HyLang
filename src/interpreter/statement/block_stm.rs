use std::rc::Rc;

use crate::interpreter::{library::{exception::Exception, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct BlockStmImpl{
    pub statements: Vec<Box<Statement>>,
}

impl  BlockStmImpl {
    pub fn interpret(&self,context:Rc<Context>)-> Result<Task, Exception> {
        for stm in self.statements.iter(){
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
    pub fn new() -> BlockStmImpl{
        BlockStmImpl{statements : vec![]}
    }
    pub fn add(&mut self , statement: Box<Statement> ){
        self.statements.push(statement);
    }
}
