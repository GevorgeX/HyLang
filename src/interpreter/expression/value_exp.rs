use std::rc::Rc;

use crate::interpreter::library::{exception::Exception, object::ReferenceToObject, Context};

#[derive(Clone)]
pub struct ValueExp{
    name: String,
}

impl super::Expression for ValueExp {
    fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        context.get_object(&self.name)
    }
}

impl ValueExp {
    pub fn new(name: String ) -> ValueExp {
        ValueExp{name}
    }
}