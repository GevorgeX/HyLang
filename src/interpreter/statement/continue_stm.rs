use crate::interpreter::{library::exception::Exception, task::Task};


#[derive(Clone)]
pub struct ContinueStmImpl{

}

impl  ContinueStmImpl{
    pub fn interpret(&self) -> Result<Task, Exception> {
        Ok(Task::Continue)
    }
    pub fn new()->ContinueStmImpl {
        ContinueStmImpl{}
    } 
}

