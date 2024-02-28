use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::Context, task::Task};

pub struct DefineVariableStm{
    name: String,
    value: Box<dyn Expression>,
}

impl super::Statement for DefineVariableStm {
    fn interpret(&self, context:Rc<Context>) -> Task {

        match &*context {
            Context::LocalContext(local) => local.define_variable(self.name.clone(), self.value.evaluate(context.clone())),
            _=> panic!("Cant define var in this context")

        }
        Task::DefineReference(self.name.clone())
    }
}

impl DefineVariableStm {
    pub fn new( name: String,value :Box<dyn Expression> ) -> DefineVariableStm {
        DefineVariableStm{name , value}
    }
}