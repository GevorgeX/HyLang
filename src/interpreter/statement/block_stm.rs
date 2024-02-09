use std::cell::RefCell;

use crate::interpreter::{task::Task, MemRef};

use super::Statement;

pub struct BlockStm{
    statements: RefCell<Vec<Box<dyn Statement>>>,
    local_variables: RefCell<Vec<String>>,
    mem: MemRef,
}

impl Statement for BlockStm {
    fn interpret(&self)-> Task {
        let mut res = Task::Default;
        for stm in self.statements.borrow().iter(){
            res = stm.interpret();

            match res {
                Task::DefineReference(var_name) =>{
                    res = Task::Default;
                    self.local_variables.borrow_mut().push(var_name);

                }
                _ => () ,
            }
        }
        for var in self.local_variables.borrow().iter(){
            self.mem.delete_variable(var);
        }

        res
    }
}

impl BlockStm {
    pub fn new(mem:MemRef) -> BlockStm{
        BlockStm{statements : RefCell::new(vec![]), local_variables: RefCell::new(vec![]), mem}
    }
    pub fn add(&self , statement: Box<dyn Statement> ){
        self.statements.borrow_mut().push(statement);
    }
}