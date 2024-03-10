use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

#[derive(Clone)]
pub struct PrintStmImpl{
    pub value: Box<Expression>,
    
}

impl PrintStmImpl {
    pub fn interpret(&self,context:Rc<Context>) -> Result<Task, Exception>{
        println!("{}", self.value.evaluate(context).unwrap().to_string());

        Ok(Task::Default)
    }
    pub fn new(value :Box<Expression> ) -> PrintStmImpl {
        PrintStmImpl{ value }
    }
}