use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{object, Context}, task::Task};

#[derive(Clone)]
pub struct FunctionCallStm{
    name: String,
    arguments: Vec<Box<dyn Expression> >,
}

impl super::Statement for FunctionCallStm {
    fn interpret(&self, context: Rc<Context>) -> Task{
        let obj = context.get_object(&self.name);
      
        
        match &*obj {
            object::Object::FunctionObject(func) => func.call(self.arguments.clone(), context) ,
            _ => panic!("Cant call {}", obj.to_string())
        }
        Task::Default
    }
}

impl FunctionCallStm {
    pub fn new( name: String,arguments: Vec<Box<dyn Expression>> ) -> FunctionCallStm {
        FunctionCallStm{name, arguments}
    }
}