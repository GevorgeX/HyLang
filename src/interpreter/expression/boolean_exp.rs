use crate::interpreter::library::{create_object, Context, ReferenceToObject};
use crate::interpreter::library::object::Object;

pub struct BooleanExp {
    value: bool,
}

impl super::Expression for BooleanExp {
    fn evaluate(&self,context:&Context) -> ReferenceToObject {
        let val = Object::Bool(self.value);
        create_object(val)

    }
}

impl BooleanExp {
    pub fn new(value : bool, ) -> BooleanExp {
        BooleanExp {value}
    }
}