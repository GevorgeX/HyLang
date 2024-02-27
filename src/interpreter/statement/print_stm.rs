use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::Context, task::Task};

pub struct PrintStm{
    value: Box<dyn Expression>,
    
}

impl super::Statement for PrintStm {
    fn interpret(&self,context:Rc<Context>) -> Task{
        println!("{}", self.value.evaluate(context).to_string());

        Task::Default
    }
}

impl PrintStm {
    pub fn new(value :Box<dyn Expression> ) -> PrintStm {
        PrintStm{ value }
    }
}