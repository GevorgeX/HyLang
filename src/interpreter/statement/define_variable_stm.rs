use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

#[derive(Clone)]
pub struct DefineVariableStmImpl{
    pub name: String,
    pub value: Box<Expression>,
}

impl  DefineVariableStmImpl {
    pub fn interpret(&self, context:Rc<Context>) -> Result<Task, Exception> {

        match &*context {
            Context::LocalContext(local) => match local.define_variable(self.name.clone(),
             self.value.evaluate(context.clone()).unwrap() ){
                Ok(_) => Ok(Task::Default),
                Err(e) => Err(e),        
            },
            _=> Err(Exception::cant_def_in_cont(self.name.clone()))

        }
    }
    pub fn new( name: String,value :Box<Expression> ) -> DefineVariableStmImpl {
        DefineVariableStmImpl{name , value}
    }
}