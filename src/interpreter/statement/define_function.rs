use std::rc::Rc;

use crate::interpreter::{library::{function::Function, object::{create_object, Object}, Context},task::Task};

use super::Statement;

#[derive(Clone)]
pub struct DefineFunctionStm{
    name: String,
    body: Box<dyn Statement>,
    args: Vec<String>
}

impl super::Statement for DefineFunctionStm {
    fn interpret(&self, context:Rc<Context>) -> Task {
        match &*context {
            Context::ModuleContext(cont) => {
                let func = Function::new(context.clone(), self.args.clone(), self.body.clone());
                let func = Object::FunctionObject(func);
                cont.define_function(self.name.clone(), create_object(func) )
            },
            _=> panic!("Cant define {} function in this context" , self.name)
        }
        Task::Default
    }
    
}

impl DefineFunctionStm {
    pub fn new( name: String,args:Vec<String> ,body :Box<dyn Statement>) -> DefineFunctionStm {
        DefineFunctionStm{name ,args , body}
    }
}