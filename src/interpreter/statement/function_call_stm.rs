use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object, Context}, task::Task};

#[derive(Clone)]
pub struct FunctionCallStm{
    name: String,
    arguments: Vec<Box<Expression> >,
}

impl super::Statement for FunctionCallStm {
    fn interpret(&self, context: Rc<Context>) -> Result<Task, Exception>{
        let obj = match context.get_object(&self.name) {
            Ok(o) => o,
            Err(e) => return Err(e),
        }; 
        
        match &*obj {
            object::Object::FunctionObject(func) => match func.call(self.arguments.clone(), context){
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            _ => return Err(Exception::is_not_func(self.name.clone()))
        }
        Ok(Task::Default)
    }
}

impl FunctionCallStm {
    pub fn new( name: String,arguments: Vec<Box<Expression>> ) -> FunctionCallStm {
        FunctionCallStm{name, arguments}
    }
}