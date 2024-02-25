use std::rc::Rc;

use self::{local_context::LocalContext, object::Object};

pub mod object;
pub mod object_utils;
mod local_context;
mod module_context;
mod function;

pub enum Context<'a> {
    LocalContext(local_context::LocalContext<'a>),
}

pub type ReferenceToObject<'a> = Rc<Object<'a>>;

pub fn create_object(val:Object) -> ReferenceToObject {
    Rc::new(val)
}

impl<'a> Context<'a> {
    pub fn new_local_context(parent: Option<&'a Context>) -> Context<'a>{
        Context::LocalContext(LocalContext::new(parent))
    }
}