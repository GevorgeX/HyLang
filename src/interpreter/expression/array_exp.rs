use core::fmt;
use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::Context;
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};

use super::Expression;
#[derive(Clone)]

pub struct ArrayExpImpl{
    pub value: Vec<Box<Expression>>,
}

impl ArrayExpImpl {
    pub fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let mut array = vec![];
        for val in &self.value  {

            let value = match val.evaluate(context.clone()){
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            array.push(value) ;
        }
        let array = create_object(Object::Array(array));

        Ok(array)
    }
    pub fn new(value:Vec<Box<Expression>>) ->Self{
        Self{value}
    }
}
