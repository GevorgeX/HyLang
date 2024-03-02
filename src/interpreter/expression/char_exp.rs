use std::rc::Rc;

use crate::interpreter::library::{create_object, Context, ReferenceToObject};
use crate::interpreter::library::object::Object;

#[derive(Clone)]
pub struct CharExp{
    value: String,
}

impl super::Expression for CharExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        let chars:Vec<char> = self.value.chars().collect();
        let val = Object::Char(*chars.first().unwrap());
        create_object(val)
    }
}

impl CharExp {
    pub fn new(value : String) -> CharExp {
        CharExp{value}
    }
}