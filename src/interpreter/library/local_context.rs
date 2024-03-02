use std::{cell::RefCell, collections::HashMap, rc::Weak};

use super::{Context, ReferenceToObject};

pub struct LocalContext{
    references: RefCell<HashMap<String , ReferenceToObject>>,
    pub parent: Option<Weak<Context>>
}


impl LocalContext {

    pub fn define_variable(&self, name:String, refer: ReferenceToObject) {
        let mut mem = self.references.borrow_mut();

        if !mem.contains_key(&name){
            mem.insert(name ,refer);
        }
        else {
            panic!("This variable is already defined");
        }
    }

    pub fn change_variable(&self, name:String, refer: ReferenceToObject ){
        let mut mem = self.references.borrow_mut();

        if mem.contains_key(&name){
            mem.insert(name ,refer);
        }
        else if let Some(par) = &self.parent  {
            match &*par.upgrade().unwrap() {
                Context::LocalContext(cont) => cont.change_variable(name, refer),
                _ => panic!("This variable isn't  define")
            }
        }
        else{
            panic!("This variable isn't  define")
        }
    }
    pub  fn delete_variable(&self, name: &String) {
        self.references.borrow_mut().remove(name);
    }

    pub fn get_variable(&self, name: &String) -> Option<ReferenceToObject> {
        if  let Some(val) = self.references.borrow().get(name) {
            Some(val.clone())
        }
        else{
            None
        }
    }

    pub fn new(parent: Option<Weak<Context>> )->LocalContext {
        LocalContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}