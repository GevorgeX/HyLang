use std::{cell::RefCell, collections::HashMap, rc::Weak};

use super::{Context, ReferenceToObject};

pub struct ModuleContext{
    references: RefCell<HashMap<String , ReferenceToObject>>,
    parent: Option<Weak<Context>>
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


    pub fn get_function(&self, name: &String) -> ReferenceToObject {
        self.references.borrow().get(name).unwrap_or_else(||{
            panic!("Function doesnt defined");
        }).clone()
    }

    pub fn new(parent: Option<Weak<Context>> )->ModuleContext {
        ModuleContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}