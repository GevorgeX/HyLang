use crate::interpreter::library::{create_object, ReferenceToObject};
use crate::interpreter::library::object::Object;
use crate::interpreter::MemRef;

pub struct NumberExp{
    value: i32,
    mem: MemRef
}

impl super::Expression for NumberExp {
    fn evaluate(&self) -> ReferenceToObject {
        let val = Object::Number(self.value);
        create_object(val)
    }
}

impl NumberExp {
    pub fn new(value : i32 , mem:MemRef) -> NumberExp {
        NumberExp{value ,mem}
    }
}