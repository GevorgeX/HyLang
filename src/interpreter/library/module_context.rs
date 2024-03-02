use std::{cell::RefCell, collections::HashMap, rc::Weak};

use super::{exception::Exception, object::ReferenceToObject, Context};

pub struct ModuleContext{
    references: RefCell<HashMap<String , ReferenceToObject>>,
    pub parent: Option<Weak<Context>>
}


impl ModuleContext {

    pub fn define_function(&self, name:String, func: ReferenceToObject) -> Result<(),Exception> {
        let mut mem = self.references.borrow_mut();

        if !mem.contains_key(&name){
            mem.insert(name ,func);
            Ok(())
        }
        else {
            Err(Exception::new_func_alr_def(name))
        }
    }


    pub fn get_function(&self, name: &String) -> Result<ReferenceToObject, Exception> {
        if let Some(val) = self.references.borrow().get(name){
            Ok(val.clone())
        }
        else {
            Err(Exception::new_object_does_Exit(name.clone()))
        }
    }

    pub fn new(parent: Option<Weak<Context>> )->ModuleContext {
        ModuleContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}