#[derive(Debug)]
pub struct Exception{
    exc_type: ExceptionType,
    pub message: String
}
#[derive(Debug)]
enum ExceptionType {
    ObjectDoesExit{obj_name:String},
    RequireSymbol{symbol:String},
    UnknowWord,
    VariableAlreadyDefine{var_name:String},
    FunctionAlreadyDefine{var_name:String},
    CantDefineInThisContext{var_name:String},
    IsNotFunction{var_name:String},
    IsNotA{var_name:String, target:String},
}

impl Exception {
    pub fn object_does_exit(obj_name:String)->Self {
        Exception{
            message: format!("{} անունով էակ գոյ․ չունի" , obj_name).to_string(),
            exc_type:ExceptionType::ObjectDoesExit { obj_name}
        }
    }
    pub fn require_symbol(symbol:String)->Self {
        Exception{
            message: format!("անհրաժեշտ է `{}`" , symbol).to_string(),
            exc_type:ExceptionType::RequireSymbol { symbol}
        }
    }
    pub fn unknow_word()->Self {
        Exception{
            message: "էս ինչա".to_string(),
            exc_type:ExceptionType::UnknowWord { }
        }
    }
    pub fn func_alr_def(var_name:String)->Self {
        Exception{
            message: format!("{} անունով ֆունկցիա արդեն հայտարարված է", var_name).to_string(),
            exc_type:ExceptionType::FunctionAlreadyDefine { var_name}
        }
    }
    pub fn var_alr_def(var_name:String)->Self {
        Exception{
            message: format!("{} անունով փոփոխական արդեն հայտարարված է", var_name).to_string(),
            exc_type:ExceptionType::VariableAlreadyDefine { var_name}
        }
    }
    pub fn cant_def_in_cont(var_name:String )->Self {
        Exception{
            message: format!("՝{}՝  չէք կարող հայտարարել այս տեղ", var_name).to_string(),
            exc_type:ExceptionType::CantDefineInThisContext { var_name}
        }
    }
    pub fn is_not_func(var_name:String )->Self {
        Exception{
            message: format!("{} ֆունկցիա չէ", var_name).to_string(),
            exc_type:ExceptionType::IsNotFunction { var_name}
        }
    }
    pub fn is_not_a(var_name:String , target:String )->Self {
        Exception{
            message: format!("{} {} չէ", var_name , target).to_string(),
            exc_type:ExceptionType::IsNotA { var_name, target}
        }
    }
}