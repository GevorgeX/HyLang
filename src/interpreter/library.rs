use std::rc::{Rc, Weak};

use self::object::Object;

pub mod object;
pub mod object_utils;
pub mod function;

mod local_context;
mod module_context;

pub enum Context {
    LocalContext(local_context::LocalContext),
    ModuleContext(module_context::ModuleContext)
}

pub type ReferenceToObject = Rc<Object>;

pub fn create_object(val:Object) -> ReferenceToObject {
    Rc::new(val)
}

impl Context {
    pub fn new_local_context(parent: Option<Weak<Context>>) -> Rc<Context>{
        Rc::new(Context::LocalContext(local_context::LocalContext::new(parent)))
    }
    pub fn new_module_context(parent: Option<Weak<Context>>) -> Rc<Context>{
        Rc::new(Context::ModuleContext(module_context::ModuleContext::new(parent)))
    }
}