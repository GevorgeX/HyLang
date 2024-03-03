use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct ReturnStm{
    value: Box<dyn Expression>
}

impl Statement for ReturnStm{
    fn interpret(&self, context:Rc<Context>) -> Result<Task, Exception> {
        match self.value.evaluate(context){
            Ok(o) => Ok(Task::Return(o)),
            Err(e) => Err(e),
        }
    }
}

impl ReturnStm {
    pub fn new(value: Box<dyn Expression>)->ReturnStm {
        ReturnStm{value}
    }
}