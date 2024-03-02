use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{object::Object, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct WhileStm{
    condition: Box<dyn Expression>,
    while_statement: Box<dyn Statement>,
}



impl Statement for WhileStm {
    fn interpret(&self,parent_context:Rc<Context>)->Task {
        let mut res = Task::Default;
        let context = Context::new_local_context(Some(Rc::downgrade(&parent_context)));

        loop {
            if let Object::Bool(cond) = *self.condition.evaluate(parent_context.clone()){
                if cond{
                    let task = self.while_statement.interpret(context.clone());
        
                    match task {
                        Task::Break => break,
                        Task::Continue => continue,
                        _=> res = task,
                    }
                }else { break }
            }
            else{
                panic!("{} is not a bool" ,self.condition.evaluate(parent_context).to_string())
            }
        }
        res
    }
}

impl WhileStm {
    pub fn new(condition: Box<dyn Expression>,
        while_statement: Box<dyn Statement>,) -> WhileStm {
        
            WhileStm{condition, while_statement}
    }
}