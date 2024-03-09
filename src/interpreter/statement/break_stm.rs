use crate::interpreter::{library::exception::Exception, task::Task};


#[derive(Clone)]
pub struct BreakStmImpl{

}

impl BreakStmImpl{
    pub fn interpret(&self) -> Result<Task, Exception> {
        Ok(Task::Break)
    }
    pub fn new()->BreakStmImpl {
        BreakStmImpl{}
    }
}
