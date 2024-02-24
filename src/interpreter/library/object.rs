use super::ReferenceToObject;

#[derive(Clone)]
pub enum Object {
    Char(char),
    Number(i32),
    Bool(bool),
    Array(Vec<ReferenceToObject>)
    // Class()
}

impl Object {
    pub fn to_string(&self) -> String{
        match self {
            Object::Char(c) => format!("Char({})",c ).to_string(),
            Object::Number(n) => format!("Number({})",n ).to_string(),
            Object::Bool(b) => format!("Bool({})",b ).to_string(),
            Object::Array(a) => format!("Array({})",a.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ")).to_string(),
        }
    }
}

// impl Drop for Object {
//     fn drop(&mut self) {
//         println!("{} was deleted",self.to_string());
//     }
// }