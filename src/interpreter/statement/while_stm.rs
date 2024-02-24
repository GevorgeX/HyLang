use crate::interpreter::{expression::Expression, library::{object::Object, Context}, task::Task};

use super::Statement;

pub struct WhileStm{
    condition: Box<dyn Expression>,
    while_statement: Box<dyn Statement>,
}



impl Statement for WhileStm {
    fn interpret(&self,parent_context:&Context)->Task {
        let mut res = Task::Default;
        let context = Context::new_local_context(Some(parent_context));

        loop {
            if let Object::Bool(cond) = *self.condition.evaluate(parent_context){
                if cond{
                    let task = self.while_statement.interpret(&context);
        
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