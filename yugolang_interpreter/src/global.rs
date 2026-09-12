use common_macros::hash_map;
use super::{typing::{Value, Function, FunctionType, Priority}, state::{Frame, Variable}};
use yugolang_parser::Scope;

pub fn global_frame() -> Frame {
    Frame::new(hash_map!{
        "let".into() => fn_let()
    })
}

fn function(type_info:FunctionType, definition:Scope) -> Variable {
    Variable::new(Value::Function(Function::new(type_info, definition)))
}

fn fn_let() -> Variable {
    function(FunctionType::new_r(Priority::new(0), todo!(), todo!()), todo!())
}

