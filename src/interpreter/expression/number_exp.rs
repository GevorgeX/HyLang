use std::rc::Rc;

use crate::interpreter::library::{create_object, Context, ReferenceToObject};
use crate::interpreter::library::object::Object;

#[derive(Clone)]
pub struct NumberExp{
    value: i32,
}

impl super::Expression for NumberExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        let val = Object::Number(self.value);
        create_object(val)
    }
}

impl NumberExp {
    pub fn new(value : i32) -> NumberExp {
        NumberExp{value}
    }
}