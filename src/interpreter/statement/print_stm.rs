use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, Context}, task::Task};

#[derive(Clone)]
pub struct PrintStm{
    value: Box<dyn Expression>,
    
}

impl super::Statement for PrintStm {
    fn interpret(&self,context:Rc<Context>) -> Result<Task, Exception>{
        println!("{}", self.value.evaluate(context).unwrap().to_string());

        Ok(Task::Default)
    }
}

impl PrintStm {
    pub fn new(value :Box<dyn Expression> ) -> PrintStm {
        PrintStm{ value }
    }
}