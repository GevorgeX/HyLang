use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::Context;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};
#[derive(Clone)]

pub struct NumberExpImpl{
    value: i32,
}

impl NumberExpImpl {
    pub fn evaluate(&self,_context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let val = Object::Number(self.value);
        Ok(create_object(val))
    }
    pub fn new(value:i32) ->Self{
        Self{value}
    }
}