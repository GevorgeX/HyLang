use std::rc::Rc;

use crate::interpreter::library::exception::Exception;
use crate::interpreter::library::{ Context};
use crate::interpreter::library::object::{create_object, Object, ReferenceToObject};

use super::Expression;

#[derive(Clone)]
pub struct ArrayExp{
    value: Vec<Box<dyn Expression>>,
}

impl super::Expression for ArrayExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        let mut array = vec![];
        for val in &self.value  {
            array.push(val.evaluate(context.clone()).unwrap()) ;
        }
        let array = create_object(Object::Array(array));

        Ok(array)
    }
}

impl ArrayExp {
    pub fn new(value : Vec<Box<dyn Expression>>) -> ArrayExp {
        ArrayExp{value}
    }
}