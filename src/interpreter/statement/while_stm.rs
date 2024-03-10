use std::rc::Rc;

use crate::interpreter::{expression::Expression, library::{exception::Exception, object::Object, Context}, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct WhileStmImpl{
    pub condition: Box<Expression>,
    pub while_statement: Box<Statement>,
}



impl  WhileStmImpl {
    pub fn interpret(&self,parent_context:Rc<Context>)->Result<Task, Exception> {
        loop {
            match self.condition.evaluate(parent_context.clone()) {
                Ok(cond) => match &*cond {
                    Object::Bool(val) => if *val == false {
                        break;
                    }
                    _=> return Err(Exception::is_not_a(cond.to_string(), "bool".to_string()))
                },
                Err(e) => return Err(e),
            }
            let context = Context::new_local_context(Some(Rc::downgrade(&parent_context)));
            match self.while_statement.interpret(context.clone()) {
                Ok(o) => match o {
                    Task::Default => (),
                    Task::Break => break,
                    Task::Continue => continue,
                    Task::Return(_) => return Ok(o),
                },
                Err(e) => return Err(e),
            }

        }
        Ok(Task::Default)
    }
    pub fn new(condition: Box<Expression>,
        while_statement: Box<Statement>,) -> WhileStmImpl {
        WhileStmImpl{condition, while_statement}
    }
}