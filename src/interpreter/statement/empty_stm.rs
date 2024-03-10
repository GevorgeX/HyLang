use std::rc::Rc;

use crate::interpreter::{library::{exception::Exception, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct EmptyStmImpl{
    pub body: Box<Statement>,
}

impl EmptyStmImpl {
    pub fn interpret(&self,context:Rc<Context>)-> Result<Task, Exception> {
        let cont = Context::new_local_context(Some(Rc::downgrade(&context)));

        self.body.interpret(cont.clone())
    }
    pub fn new(body:Box<Statement>) -> EmptyStmImpl{
        EmptyStmImpl{body }
    }
}
