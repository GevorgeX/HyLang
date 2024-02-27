use std::rc::{Rc, Weak};

use self::{local_context::LocalContext, object::Object};

pub mod object;
pub mod object_utils;
mod local_context;
mod module_context;
mod function;

pub enum Context {
    LocalContext(local_context::LocalContext),
}

pub type ReferenceToObject = Rc<Object>;

pub fn create_object(val:Object) -> ReferenceToObject {
    Rc::new(val)
}

impl Context {
    pub fn new_local_context(parent: Option<Weak< Context>>) -> Rc<Context>{
        Rc::new(Context::LocalContext(LocalContext::new(parent)))
    }
}