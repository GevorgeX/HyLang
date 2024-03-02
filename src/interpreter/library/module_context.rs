use std::{cell::RefCell, collections::HashMap, rc::Weak};

use super::{Context, ReferenceToObject};

pub struct ModuleContext{
    references: RefCell<HashMap<String , ReferenceToObject>>,
    pub parent: Option<Weak<Context>>
}


impl ModuleContext {

    pub fn define_function(&self, name:String, func: ReferenceToObject) {
        let mut mem = self.references.borrow_mut();

        if !mem.contains_key(&name){
            mem.insert(name ,func);
        }
        else {
            panic!("This variable is already defined");
        }
    }


    pub fn get_function(&self, name: &String) -> Option<ReferenceToObject> {
        if let Some(val) = self.references.borrow().get(name){
            Some(val.clone())
        }
        else {
            None
        }
    }

    pub fn new(parent: Option<Weak<Context>> )->ModuleContext {
        ModuleContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}