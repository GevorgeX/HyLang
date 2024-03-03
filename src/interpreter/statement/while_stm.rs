use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::Object, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct WhileStm{
    condition: Box<dyn Expression>,
    while_statement: Box<dyn Statement>,
}



impl Statement for WhileStm {
    fn interpret(&self,parent_context:Rc<Context>)->Result<Task, Exception> {
        let mut res = Task::Default;
        let context = Context::new_local_context(Some(Rc::downgrade(&parent_context)));

        loop {
            let object = match self.condition.evaluate(parent_context.clone()){
                Ok(o) => o,
                Err(e) => return Err(e),
            };

            if let Object::Bool(cond) = *object{
                if cond{
                    let task =match self.while_statement.interpret(context.clone()){
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    
                    match task {
                        Task::Break => break,
                        Task::Continue => continue,
                        _=> res = task,
                    }
                }else { break }
            }
            else{
                return Err(Exception::is_not_a(object.to_string(), "Bool".to_string()));
            }
        }
        Ok(res)
    }
}

impl WhileStm {
    pub fn new(condition: Box<dyn Expression>,
        while_statement: Box<dyn Statement>,) -> WhileStm {
        
            WhileStm{condition, while_statement}
    }
}