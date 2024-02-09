use crate::interpreter::{expression::Expression, task::Task, MemRef};

pub struct AssignmentStm{
    name: String,
    value: Box<dyn Expression>,
    mem: MemRef,
}

impl super::Statement for AssignmentStm {
    fn interpret(&self) -> Task{
        self.mem.change_variable(self.name.clone(), self.value.evaluate());
        Task::Default
    }
}

impl AssignmentStm {
    pub fn new( name: String,value :Box<dyn Expression> , mem:MemRef) -> AssignmentStm {
        AssignmentStm{name , value , mem}
    }
}