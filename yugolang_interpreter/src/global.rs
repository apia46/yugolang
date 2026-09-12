use common_macros::hash_map;
use super::{typing::{Value, Function, Type, FunctionType, Priority}, state::{Frame, Variable}};
use yugolang_parser::Scope;
use yugolang_parser_macro::parse;

pub fn global_frame() -> Frame {
    Frame::new(hash_map!{
        "+".into() => fn_add()
    })
}

fn function(type_info:FunctionType, definition:Scope) -> Variable {
    Variable::new(Value::Function(Function::new(type_info, definition)))
}

fn fn_add() -> Variable {
    function(FunctionType::curry_lr(Priority::new(0), Type::Int, Type::Int, Type::Int),parse!("hi"))
}

