use crate::interpreter::memory::ReferenceToObject;
use crate::interpreter::memory::object::Object;
use crate::interpreter::MemRef;

pub struct CharExp{
    value: String,
    mem:MemRef
}

impl super::Expression for CharExp {
    fn evaluate(&self) -> ReferenceToObject {
        let chars:Vec<char> = self.value.chars().collect();
        let val = Object::Char(*chars.first().unwrap());
        self.mem.create_object(val)
    }
}

impl CharExp {
    pub fn new(value : String, mem:MemRef) -> CharExp {
        CharExp{value ,mem}
    }
}