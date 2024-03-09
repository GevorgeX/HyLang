use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};
#[derive(Clone)]

pub struct CharExpImpl{
    value: String,
}

impl CharExpImpl {
    pub fn evaluate(&self) -> Result<ReferenceToObject,Exception> {
        let chars:Vec<char> = self.value.chars().collect();
        let val = Object::Char(*chars.first().unwrap());
        Ok(create_object(val))
    }
    pub fn new(value:String) ->Self{
        Self{value}
    }
}

