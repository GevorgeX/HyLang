use crate::interpreter::task::Task;

use super::Statement;

pub struct BreakStm{

}

impl Statement for BreakStm{
    fn interpret(&self) -> Task {
        Task::Break
    }
}

impl BreakStm {
    pub fn new()->BreakStm {
        BreakStm{}
    }
}