use super::{function::Function, ReferenceToObject};

#[derive(Clone)]
pub enum Object<'a> {
    Char(char),
    Number(i32),
    Bool(bool),
    FunctionObject(Function<'a>),

    Array(Vec<ReferenceToObject<'a>>),
    
}

impl<'a> Object<'a> {
    pub fn to_string(&self) -> String{
        match self {
            Object::Char(c) => format!("Char({})",c ).to_string(),
            Object::Number(n) => format!("Number({})",n ).to_string(),
            Object::Bool(b) => format!("Bool({})",b ).to_string(),
            Object::Array(a) => format!("Array({})",a.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ")).to_string(),
            Object::FunctionObject(_) => todo!(),
        }
    }
}

// impl Drop for Object {
//     fn drop(&mut self) {
//         println!("{} was deleted",self.to_string());
//     }
// }