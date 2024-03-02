use std::rc::Rc;

use crate::interpreter::library::{Context, ReferenceToObject};

#[derive(Clone)]
pub struct ValueExp{
    name: String,
}

impl super::Expression for ValueExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        context.get_object(&self.name)
    }
}

impl ValueExp {
    pub fn new(name: String ) -> ValueExp {
        ValueExp{name}
    }
}