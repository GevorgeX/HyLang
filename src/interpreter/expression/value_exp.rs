use crate::interpreter::MemRef;
use crate::interpreter::library::ReferenceToObject;

pub struct ValueExp{
    name: String,
    mem: MemRef
}

impl super::Expression for ValueExp {
    fn evaluate(&self) -> ReferenceToObject {
        self.mem.get_variable(&self.name)
    }
}

impl ValueExp {
    pub fn new(name: String , mem: MemRef) -> ValueExp {
        ValueExp{name ,mem}
    }
}