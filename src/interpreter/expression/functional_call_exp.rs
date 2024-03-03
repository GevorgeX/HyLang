use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::{self, ReferenceToObject}, Context}, task::Task};

#[derive(Clone)]
pub struct FunctionCallExp{
    name: String,
    arguments: Vec<Box<dyn Expression>>,
}

impl super::Expression for FunctionCallExp {
    fn evaluate(&self, context: Rc<Context>) -> Result<ReferenceToObject, Exception>{
        let obj = match context.get_object(&self.name) {
            Ok(o) => o,
            Err(e) => return Err(e),
        }; 
        
        match &*obj {
            object::Object::FunctionObject(func) => match func.call(self.arguments.clone(), context){
                Ok(val) => Ok(val),
                Err(e) => Err(e),
            },
            _ => Err(Exception::is_not_func(self.name.clone()))
        }
    }
}

impl FunctionCallExp {
    pub fn new( name: String,arguments: Vec<Box<dyn Expression>> ) -> FunctionCallExp {
        FunctionCallExp{name, arguments}
    }
}