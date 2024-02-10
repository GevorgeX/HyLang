pub mod object;
pub mod object_utils;
mod context;

use std::borrow::Borrow;
use std::collections::hash_map::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use self::object::Object;

pub type ReferenceToObject = Rc<Object>;

pub struct LocalContext{
    pub references: RefCell<HashMap<String , ReferenceToObject>>,
    pub parent: Option<Box<Self>>
}

pub fn create_object(val:Object) -> ReferenceToObject {
    Rc::new(val)
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
        else if let Some(par) = self.parent  {
            par.change_variable(name, refer);
        }
        else{
            panic!("This variable isn't  define")
        }
    }
    pub  fn delete_variable(&self, name: &String) {
        self.references.borrow_mut().remove(name);
    }

    pub fn get_variable(&self, name: &String) -> ReferenceToObject {
        self.references.borrow().get(name).unwrap_or_else(||{
            match self.parent {
                Some(par) => &par.get_variable(name),
                None => panic!("This variable isn't  define"),
            }
        }).clone()
    }

    pub fn new(parent: Option<Box<Self>> )->LocalContext {
        LocalContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}