use std::rc::Rc;

use crate::interpreter::library::{exception::Exception, object::ReferenceToObject, Context};
#[derive(Clone)]

pub struct ValueExpImpl{
    name: String,
}

impl ValueExpImpl {
    pub fn evaluate(&self,context:Rc<Context>) -> Result<ReferenceToObject,Exception> {
        context.get_object(&self.name)
    }
    pub fn new(name:String) -> Self{
        Self { name }
    }
}