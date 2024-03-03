use std::rc::{Rc, Weak};

use self::{exception::Exception, object::ReferenceToObject};


pub mod object;
pub mod object_utils;
pub mod function;
pub mod exception;

mod local_context;
mod module_context;

pub enum Context {
    LocalContext(local_context::LocalContext),
    ModuleContext(module_context::ModuleContext)
}


impl Context {
    pub fn get_object(&self, name:&String) -> Result<ReferenceToObject,Exception>{
        match self {
            Context::LocalContext(local) => {
                let req = local.get_variable(name);
                if req.is_ok(){
                    return req;
                }
                else{
                    if let Some(par) = &local.parent{
                        return par.upgrade().unwrap().get_object(name);
                    }
                    else{
                        Err(Exception::object_does_exit(name.clone()))
                    }
                }
            },
            Context::ModuleContext(modul)=>{
                return modul.get_function(name);
            },

        }
    }
   
    pub fn new_local_context(parent: Option<Weak<Context>>) -> Rc<Context>{
        Rc::new(Context::LocalContext(local_context::LocalContext::new(parent)))
    }
    pub fn new_module_context(parent: Option<Weak<Context>>) -> Rc<Context>{
        Rc::new(Context::ModuleContext(module_context::ModuleContext::new(parent)))
    }
}

