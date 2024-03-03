use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::Context;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};

#[derive(Clone)]

pub struct NumberExp{
    value: i32,
}

impl super::Expression for NumberExp {
    fn evaluate(&self,_context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let val = Object::Number(self.value);
        Ok(create_object(val))
    }
}

impl NumberExp {
    pub fn new(value : i32) -> NumberExp {
        NumberExp{value}
    }
}