use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::Object, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct IfElseStmImpl{
    condition: Box<Expression>,
    if_statement: Box< Statement>,
    else_statement: Option<Box<Statement>>
}



impl IfElseStmImpl {
    pub fn interpret(&self,parent_context: Rc<Context>) -> Result<Task, Exception> {
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
    pub fn new(condition: Box<Expression>,
        if_statement: Box< Statement>,
        else_statement: Option<Box<Statement>>) -> IfElseStmImpl {
        
            IfElseStmImpl{
            condition, if_statement ,else_statement
        }
    }
}