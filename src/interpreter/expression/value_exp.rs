use std::rc::Rc;

use crate::interpreter::library::{Context, ReferenceToObject};

pub struct ValueExp{
    name: String,
}

impl super::Expression for ValueExp {
    fn evaluate(&self,context:Rc<Context>) -> ReferenceToObject {
        return match &*context {
            Context::LocalContext(local) => local.get_variable(&self.name),
        }
    }
}

impl ValueExp {
    pub fn new(name: String ) -> ValueExp {
        ValueExp{name}
    }
}