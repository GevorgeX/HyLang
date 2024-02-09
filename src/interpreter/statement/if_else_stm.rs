use crate::interpreter::{expression::Expression, memory::object::Object, task::Task};

use super::Statement;

pub struct IfElseStm{
    condition: Box<dyn Expression>,
    if_statement: Box<dyn Statement>,
    else_statement: Option<Box<dyn Statement>>
}



impl Statement for IfElseStm {
    fn interpret(&self) -> Task {
        let cond = self.condition.evaluate();
        let mut res = Task::Default;
        
        if let Object::Bool(val) = *cond{
            if val == true{
                res = self.if_statement.interpret();
            }
            else if let Some(stm) = &self.else_statement{
                res = stm.interpret();
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