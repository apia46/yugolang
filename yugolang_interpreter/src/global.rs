use common_macros::hash_map;

use crate::Error;
use crate::priorities;
use crate::typing::{Value, Function, Identifier, Type, FunctionType, FunctionDefinition};
use crate::state::{State, Frame, Variable};

pub fn global_frame() -> Frame {
    Frame::new(hash_map!{
        "+".into() => fn_add(),
        "-".into() => fn_sub(),
        "*".into() => fn_mul(),
        "/".into() => fn_div(),
    })
}

fn function(type_info:FunctionType, definition:FunctionDefinition) -> Variable {
    Variable::new(Value::Function(Function::new(type_info, definition)))
}

fn get_arg_int<'a>(s:&'a State, name: &'static str) -> Result<&'a i64, Error> {
    let Value::Int(arg) = s.get_variable(name).ok_or(Error::MissingVariableError)?.get() else { return Err(Error::TypeError) };
    Ok(arg)
}

fn fn_add() -> Variable {
    function(FunctionType::curry_lr(priorities::binary_op::ADD_SUB, Type::Int,
        Identifier::new(Type::Int, "a"),
        Identifier::new(Type::Int, "b")),
        FunctionDefinition::Magic(Box::new(|s| {
            let a = get_arg_int(s, "a")?;
            let b = get_arg_int(s, "b")?;
            Ok(Value::Int(a+b))
    })))
}

fn fn_sub() -> Variable {
    function(FunctionType::curry_lr(priorities::binary_op::ADD_SUB,  Type::Int,
        Identifier::new(Type::Int, "a"),
        Identifier::new(Type::Int, "b")),
        FunctionDefinition::Magic(Box::new(|s| {
            let a = get_arg_int(s, "a")?;
            let b = get_arg_int(s, "b")?;
            Ok(Value::Int(a-b))
    })))
}

fn fn_mul() -> Variable {
    function(FunctionType::curry_lr(priorities::binary_op::MUL_DIV, Type::Int,
        Identifier::new(Type::Int, "a"),
        Identifier::new(Type::Int, "b")),
        FunctionDefinition::Magic(Box::new(|s| {
            let a = get_arg_int(s, "a")?;
            let b = get_arg_int(s, "b")?;
            Ok(Value::Int(a*b))
    })))
}

fn fn_div() -> Variable {
    function(FunctionType::curry_lr(priorities::binary_op::MUL_DIV, Type::Int,
        Identifier::new(Type::Int, "a"),
        Identifier::new(Type::Int, "b")),
        FunctionDefinition::Magic(Box::new(|s| {
            let a = get_arg_int(s, "a")?;
            let b = get_arg_int(s, "b")?;
            if *b == 0 { return Err(Error::DivideByZeroError) };
            Ok(Value::Int(a/b))
    })))
}

