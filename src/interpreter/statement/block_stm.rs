use std::{cell::RefCell, rc::Rc};

use crate::interpreter::{library::Context, task::Task};

use super::Statement;

pub struct BlockStm{
    statements: RefCell<Vec<Box<dyn Statement>>>,
}

impl Statement for BlockStm {
    fn interpret(&self,context:Rc<Context>)-> Task {
        let mut res: Task = Task::Default;
        for stm in self.statements.borrow().iter(){
            res = stm.interpret(context.clone());
        }
        res
    }
}

impl BlockStm {
    pub fn new() -> BlockStm{
        BlockStm{
            statements : RefCell::new(vec![]),
         }
    }
    pub fn add(&self , statement: Box<dyn Statement> ){
        self.statements.borrow_mut().push(statement);
    }
}