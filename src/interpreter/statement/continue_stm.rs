use crate::interpreter::{library::Context, task::Task};

use super::Statement;

pub struct ContinueStm{

}

impl Statement for ContinueStm{
    fn interpret(&self,context:&Context) -> Task {
        Task::Continue
    }
}

impl ContinueStm {
    pub fn new()->ContinueStm {
        ContinueStm{}
    }
}