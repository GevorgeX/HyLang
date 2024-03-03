use std::rc::Rc;

use crate::interpreter::{library::{exception::Exception, function::Function, object::{create_object, Object}, Context},task::Task};

use super::Statement;

#[derive(Clone)]
pub struct DefineFunctionStm{
    name: String,
    body: Box<dyn Statement>,
    args: Vec<String>
}

impl super::Statement for DefineFunctionStm {
    fn interpret(&self, context:Rc<Context>) -> Result<Task, Exception> {
        match &*context {
            Context::ModuleContext(cont) => {
                let func = Function::new(context.clone(), self.args.clone(), self.body.clone());
                let func = Object::FunctionObject(func);
                match cont.define_function(self.name.clone(), create_object(func)){
                    Ok(_) => Ok(Task::Default),
                    Err(e) => Err(e),
                }
            },
            _=> Err(Exception::cant_def_in_cont(self.name.clone()))
        }
        
    }
    
}

impl DefineFunctionStm {
    pub fn new( name: String,args:Vec<String> ,body :Box<dyn Statement>) -> DefineFunctionStm {
        DefineFunctionStm{name ,args , body}
    }
}