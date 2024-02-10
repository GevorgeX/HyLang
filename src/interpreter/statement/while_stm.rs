use crate::interpreter::{expression::Expression, library::object::Object, task::Task};

use super::Statement;

pub struct WhileStm{
    condition: Box<dyn Expression>,
    while_statement: Box<dyn Statement>,
}



impl Statement for WhileStm {
    fn interpret(&self)->Task {
        let mut res = Task::Default;
        loop {
            if let Object::Bool(cond) = *self.condition.evaluate(){
                if cond{
                    let task = self.while_statement.interpret();
        
                    match task {
                        Task::Break => break,
                        Task::Continue => continue,
                        _=> res = task,
                    }
                }else { break }
            }
            else{
                panic!("{} is not a bool" ,self.condition.evaluate().to_string())
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