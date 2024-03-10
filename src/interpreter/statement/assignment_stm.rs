use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

#[derive(Clone)]
pub struct AssignmentStmImpl{
    pub name: String,
    pub value: Box<Expression>,
}

impl  AssignmentStmImpl {
    pub fn interpret(&self, context: Rc<Context>) -> Result<Task, Exception>{
        match &*context {
            Context::LocalContext(local) => 
            match local.change_variable(self.name.clone(), self.value.evaluate(context.clone()).unwrap()){
                Ok(_) => Ok(Task::Default),
                Err(e) => Err(e),
            },
            _=> Err(Exception::object_does_exit(self.name.clone()))
        }
    }
    pub fn new( name: String,value :Box<Expression> ) -> AssignmentStmImpl {
        AssignmentStmImpl{name , value}
    }
}
