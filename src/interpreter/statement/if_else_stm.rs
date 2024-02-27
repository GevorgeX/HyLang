use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{object::Object, Context}, task::Task};

use super::Statement;

pub struct IfElseStm{
    condition: Box<dyn Expression>,
    if_statement: Box<dyn Statement>,
    else_statement: Option<Box<dyn Statement>>
}



impl Statement for IfElseStm {
    fn interpret(&self,parent_context: Rc<Context>) -> Task {
        let mut res = Task::Default;
        let context = Context::new_local_context(Some(Rc::downgrade(&parent_context)));
        
        let cond = self.condition.evaluate(parent_context);

        if let Object::Bool(val) = *cond{
            if val == true{
                res = self.if_statement.interpret(context);
            }
            else if let Some(stm) = &self.else_statement{
                res = stm.interpret(context);
            }
        

        }
        else{
            panic!("Cant converted {} to bool", cond.to_string() )
        }
        return res
        
    }
}

impl IfElseStm {
    pub fn new(condition: Box<dyn Expression>,
        if_statement: Box<dyn Statement>,
        else_statement: Option<Box<dyn Statement>>) -> IfElseStm {
        
            IfElseStm{
            condition, if_statement ,else_statement
        }
    }
}