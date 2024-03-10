use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::{self, ReferenceToObject}, Context}};

#[derive(Clone)]
pub struct FunctionCallExpImpl{
    pub name: String,
    pub arguments: Vec<Box<Expression>>,
}

impl FunctionCallExpImpl {
    pub fn evaluate(&self, context: Rc<Context>) -> Result<ReferenceToObject, Exception>{
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
    pub fn new(name: String,arguments: Vec<Box<Expression>>,) ->Self{
        Self{name ,arguments}
    }
}