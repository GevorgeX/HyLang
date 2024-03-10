use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};
#[derive(Clone)]

pub struct BooleanExpImpl {
    pub value: bool,
}

impl  BooleanExpImpl {
    pub fn evaluate(&self) -> Result<ReferenceToObject,Exception> {
        let val = Object::Bool(self.value);
        Ok(create_object(val))

    }
    pub fn new(value:bool) ->Self{
        Self{value}
    }
}
