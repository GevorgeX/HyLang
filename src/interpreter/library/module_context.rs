use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{function::Function, Context, ReferenceToObject};

pub struct ModuleContext<'a>{
    references: RefCell<HashMap<String , ReferenceToObject<'a> >>,
    parent: Option<&'a Context<'a>>
}


impl<'a> ModuleContext<'a> {

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

    pub fn new(parent: Option<&'a Context> )->ModuleContext<'a> {
        ModuleContext{
            references: RefCell::new(HashMap::new()),
            parent
        }
    }
    
}