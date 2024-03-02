use std::rc::Rc;

use crate::interpreter::library::{create_object, Context, ReferenceToObject};
use crate::interpreter::library::object::Object;

use super::Expression;

#[derive(Clone)]
pub struct ArrayExp{
    value: Vec<Box<dyn Expression>>,
}

impl super::Expression for ArrayExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        let mut array = vec![];
        for val in &self.value  {
            array.push(val.evaluate(context.clone())) ;
        }
        let array = create_object(Object::Array(array));

        array
    }
}

impl ArrayExp {
    pub fn new(value : Vec<Box<dyn Expression>>) -> ArrayExp {
        ArrayExp{value}
    }
}