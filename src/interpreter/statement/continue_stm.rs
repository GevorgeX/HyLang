use crate::interpreter::task::Task;

use super::Statement;

pub struct ContinueStm{

}

impl Statement for ContinueStm{
    fn interpret(&self) -> Task {
        Task::Continue
    }
}

impl ContinueStm {
    pub fn new()->ContinueStm {
        ContinueStm{}
    }
}