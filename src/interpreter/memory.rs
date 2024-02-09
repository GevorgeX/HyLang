pub mod object;
pub mod object_utils;
mod context;

use std::collections::hash_map::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use self::object::Object;

pub type ReferenceToObject = Rc<Object>;

pub struct Memory{
    pub references: RefCell<HashMap<String , ReferenceToObject>>
}

impl Memory {
    pub fn create_object(&self, val:Object) -> ReferenceToObject {
         Rc::new(val)
    }
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
        else {
            panic!("This variable isn't  define");
        }
    }
    pub  fn delete_variable(&self, name: &String) {
        self.references.borrow_mut().remove(name);
    }

    pub  fn get_variable(&self, name: &String) -> ReferenceToObject {
        self.references.borrow().get(name).unwrap().clone()
    }

    pub fn new()->Memory {
        Memory{
            references: RefCell::new(HashMap::new()),
        }
    }
    
}