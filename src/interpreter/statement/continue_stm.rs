use std::rc::Rc;

use crate::interpreter::{library::{exception::Exception, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct ContinueStm{

}

impl Statement for ContinueStm{
    fn interpret(&self,_context:Rc<Context>) -> Result<Task, Exception> {
        Ok(Task::Continue)
    }
}

impl ContinueStm {
    pub fn new()->ContinueStm {
        ContinueStm{}
    }
}