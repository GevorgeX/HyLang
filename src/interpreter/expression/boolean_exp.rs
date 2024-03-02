use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::Context;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};

#[derive(Clone)]
pub struct BooleanExp {
    value: bool,
}

impl super::Expression for BooleanExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let val = Object::Bool(self.value);
        Ok(create_object(val))

    }
}

impl BooleanExp {
    pub fn new(value : bool, ) -> BooleanExp {
        BooleanExp {value}
    }
}