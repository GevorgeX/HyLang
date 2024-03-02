#[derive(Debug)]
pub struct Exception{
    exc_type: ExceptionType,
    message: String
}
#[derive(Debug)]
enum ExceptionType {
    ObjectDoesExit{obj_name:String},
    RequireSymbol{symbol:String},
    UnknowWord,
    VariableAlreadyDefine{var_name:String},
    FunctionAlreadyDefine{var_name:String},
}

impl Exception {
    pub fn new_object_does_Exit(obj_name:String)->Self {
        Exception{
            message: format!("{} անունով էակ գոյ․ չունի" , obj_name).to_string(),
            exc_type:ExceptionType::ObjectDoesExit { obj_name}
        }
    }
    pub fn new_require_symbol(symbol:String)->Self {
        Exception{
            message: format!("անհրաժեշտ է `{}`" , symbol).to_string(),
            exc_type:ExceptionType::RequireSymbol { symbol}
        }
    }
    pub fn new_unknow_word()->Self {
        Exception{
            message: "էս ինչա".to_string(),
            exc_type:ExceptionType::UnknowWord { }
        }
    }
    pub fn new_func_alr_def(var_name:String)->Self {
        Exception{
            message: format!("{} անունով ֆունկցիա արդեն հայտարարված է", var_name).to_string(),
            exc_type:ExceptionType::FunctionAlreadyDefine { var_name}
        }
    }
    pub fn new_var_alr_def(var_name:String)->Self {
        Exception{
            message: format!("{} անունով ֆունկցիա արդեն հայտարարված է", var_name).to_string(),
            exc_type:ExceptionType::VariableAlreadyDefine { var_name}
        }
    }
}