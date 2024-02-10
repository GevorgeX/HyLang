#[derive(Clone, Copy)]
pub enum Object {
    Char(char),
    Number(i32),
    Bool(bool),
    // Array()
    // Class()
}

impl Object {
    pub fn to_string(&self) -> String{
        match self {
            Object::Char(c) => format!("Char({})",c ).to_string(),
            Object::Number(n) => format!("Number({})",n ).to_string(),
            Object::Bool(b) => format!("Bool({})",b ).to_string(),
        }
    }
}

