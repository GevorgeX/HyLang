use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::Object, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct IfElseStm{
    condition: Box<dyn Expression>,
    if_statement: Box<dyn Statement>,
    else_statement: Option<Box<dyn Statement>>
}



impl Statement for IfElseStm {
    fn interpret(&self,parent_context: Rc<Context>) -> Result<Task, Exception> {
        let mut res = Task::Default;
        let context = Context::new_local_context(Some(Rc::downgrade(&parent_context)));
        
        let cond = match self.condition.evaluate(parent_context){
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        if let Object::Bool(val) = *cond{
            if val == true{
                res = match self.if_statement.interpret(context){
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };
            }
            else if let Some(stm) = &self.else_statement{
                res = match stm.interpret(context){
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };            
            }
        }
        else{
            return Err(Exception::is_not_a(cond.to_string(), "Bool".to_string()))
        }
        return Ok(res)
        
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