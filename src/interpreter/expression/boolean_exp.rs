use crate::interpreter::library::{create_object, ReferenceToObject};
use crate::interpreter::library::object::Object;
use crate::interpreter::MemRef;

pub struct BooleanExp {
    value: bool,
    mem:MemRef
}

impl super::Expression for BooleanExp {
    fn evaluate(&self) -> ReferenceToObject {
        let val = Object::Bool(self.value);
        create_object(val)

    }
}

impl BooleanExp {
    pub fn new(value : bool, mem:MemRef) -> BooleanExp {
        BooleanExp {value,mem}
    }
}