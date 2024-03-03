use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::Context;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};

#[derive(Clone)]
pub struct CharExp{
    value: String,
}

impl super::Expression for CharExp {
    fn evaluate(&self,_context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let chars:Vec<char> = self.value.chars().collect();
        let val = Object::Char(*chars.first().unwrap());
        Ok(create_object(val))
    }
}

impl CharExp {
    pub fn new(value : String) -> CharExp {
        CharExp{value}
    }
}