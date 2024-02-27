use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::Context, task::Task};

pub struct AssignmentStm{
    name: String,
    value: Box<dyn Expression>,
}

impl super::Statement for AssignmentStm {
    fn interpret(&self, context: Rc<Context>) -> Task{
        match &*context {
            Context::LocalContext(local) => 
            local.change_variable(self.name.clone(), self.value.evaluate(context.clone())),
        }
        Task::Default
    }
}

impl AssignmentStm {
    pub fn new( name: String,value :Box<dyn Expression> ) -> AssignmentStm {
        AssignmentStm{name , value}
    }
}