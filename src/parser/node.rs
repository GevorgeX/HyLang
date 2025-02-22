use std::fmt::Debug;

pub type DNode = Box<dyn Node>;

pub trait Node: Debug  {
    fn visit(&self);
}