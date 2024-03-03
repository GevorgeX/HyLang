use super::library::object::ReferenceToObject;

pub enum Task{
    Default,
    Break,
    Continue,
    Return(ReferenceToObject)
}


