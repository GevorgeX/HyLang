use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

#[derive(Clone)]
pub struct ReturnStmImpl{
    value: Box<Expression>
}

impl ReturnStmImpl{
    pub fn interpret(&self, context:Rc<Context>) -> Result<Task, Exception> {
        match self.value.evaluate(context){
            Ok(o) => Ok(Task::Return(o)),
            Err(e) => Err(e),
        }
    }
    pub fn new(value: Box<Expression>)->ReturnStmImpl {
        ReturnStmImpl{value}
    }
}