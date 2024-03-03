use std::{cell::RefCell, collections::HashMap, rc::Weak};

use super::{exception::Exception, object::ReferenceToObject, Context};

pub struct LocalContext{
    references: RefCell<HashMap<String , ReferenceToObject>>,
    pub parent: Option<Weak<Context>>
}


impl LocalContext {

    pub fn define_variable(&self, name:String, refer: ReferenceToObject) -> Result<(), Exception> {
        let mut mem = self.references.borrow_mut();

        if !mem.contains_key(&name){
            mem.insert(name ,refer);
            Ok(())
        }
        else {
            Err(Exception::var_alr_def(name))
        }
    }

    pub fn change_variable(&self, name:String, refer: ReferenceToObject ) -> Result<(), Exception>{
        let mut mem = self.references.borrow_mut();

        if mem.contains_key(&name){
            mem.insert(name ,refer);
            Ok(())
        }
        else if let Some(par) = &self.parent  {
            match &*par.upgrade().unwrap() {
                Context::LocalContext(cont) => cont.change_variable(name, refer),
                _ => Err(Exception::object_does_exit(name))
            }
        }
        else{
            Err(Exception::object_does_exit(name))
        }
    }
    pub fn get_variable(&self, name: &String) -> Result<ReferenceToObject, Exception> {
        if  let Some(val) = self.references.borrow().get(name) {
            Ok(val.clone())
        }
        else{
            Err(Exception::object_does_exit(name.clone()))
        }
    }

    pub fn new(parent: Option<Weak<Context>> )->LocalContext {
        LocalContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}