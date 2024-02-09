use crate::interpreter::memory::ReferenceToObject;
use crate::interpreter::memory::object::Object;
use crate::interpreter::MemRef;

pub struct NumberExp{
    value: i32,
    mem: MemRef
}

impl super::Expression for NumberExp {
    fn evaluate(&self) -> ReferenceToObject {
        let val = Object::Number(self.value);
        self.mem.create_object(val)
    }
}

impl NumberExp {
    pub fn new(value : i32 , mem:MemRef) -> NumberExp {
        NumberExp{value ,mem}
    }
}