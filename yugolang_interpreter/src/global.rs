use std::rc::Rc;
use common_macros::hash_map;

use crate::Error;
use crate::priorities;
use crate::typing::{Value, Function, Identifier, Type, FunctionType, FunctionDefinition, Priority};
use crate::state::{State, Frame, Variable};

pub fn global_frame() -> Frame {
    Frame::new(hash_map!{
        "+".into() => fn_add(),
        "-".into() => fn_sub(),
        "*".into() => fn_mul(),
        "/".into() => fn_div(),
        ">".into() => fn_gt(),
        "<".into() => fn_lt(),
        ">=".into() => fn_gte(),
        "<=".into() => fn_lte(),
        "if".into() => fn_if(),
        "let".into() => fn_let(),
        "=".into() => fn_set(),
        "print".into() => fn_print(),
    })
}

fn function(type_info:FunctionType, definition:FunctionDefinition) -> Variable {
    Variable::new(Value::Function(Function::new(type_info, definition).into()))
}

fn get_arg<'a>(s:&'a State, name: &'static str) -> Result<&'a Variable, Error> {
    s.get_variable(name).ok_or(Error::MissingVariableError)
}


macro_rules! get_arg {
    ($fn_name:ident, $out_type:ty, $arg:ident, $pattern:pat) => {
        fn $fn_name<'a>(s:&'a State, name: &'static str) -> Result<&'a $out_type, Error> {
            let $pattern = get_arg(s, name)?.get() else { return Err(Error::TypeError) };
            Ok($arg)
        }
    };
}

get_arg!(get_arg_int, i64, arg, Value::Int(arg));
get_arg!(get_arg_boolean, bool, arg, Value::Boolean(arg));
get_arg!(get_arg_function, Rc<Function>, arg, Value::Function(arg));
get_arg!(get_arg_string, String, arg, Value::String(arg));
get_arg!(get_arg_identifier, String, arg, Value::Identifier(arg));

macro_rules! make_binary_op {
    ($fn_name:ident, $priority:expr, $out_type:expr, $a:ident, $b:ident, $result:expr) => {
        fn $fn_name() -> Variable {
            function(FunctionType::curry_lr($priority, $out_type,
                    vec![Identifier::new(Type::Int, "a")],
                    vec![Identifier::new(Type::Int, "b")]
                ), FunctionDefinition::Magic(Box::new(|s| {
                    let $a = get_arg_int(s, "a")?;
                    let $b = get_arg_int(s, "b")?;
                    Ok($result)
            })))
        }
    };
    ($fn_name:ident, $priority:expr, $out_type:expr, $a:ident, $b:ident, $result:expr, $extra:stmt) => {
        fn $fn_name() -> Variable {
            function(FunctionType::curry_lr($priority, $out_type,
                    vec![Identifier::new(Type::Int, "a")],
                    vec![Identifier::new(Type::Int, "b")]
                ), FunctionDefinition::Magic(Box::new(|s| {
                    let $a = get_arg_int(s, "a")?;
                    let $b = get_arg_int(s, "b")?;
                    $extra
                    Ok($result)
            })))
        }
    };
}

make_binary_op!(fn_add, priorities::binary_op::ADD_SUB, Type::Int, a, b, Value::Int(a+b));
make_binary_op!(fn_sub, priorities::binary_op::ADD_SUB, Type::Int, a, b, Value::Int(a-b));
make_binary_op!(fn_mul, priorities::binary_op::MUL_DIV, Type::Int, a, b, Value::Int(a*b));
make_binary_op!(fn_div, priorities::binary_op::MUL_DIV, Type::Int, a, b, Value::Int(a/b), if *b == 0 { return Err(Error::DivideByZeroError) });
make_binary_op!(fn_gt,  priorities::binary_op::COMPARE, Type::Boolean, a, b, Value::Boolean(a>b));
make_binary_op!(fn_lt,  priorities::binary_op::COMPARE, Type::Boolean, a, b, Value::Boolean(a<b));
make_binary_op!(fn_gte, priorities::binary_op::COMPARE, Type::Boolean, a, b, Value::Boolean(a>=b));
make_binary_op!(fn_lte, priorities::binary_op::COMPARE, Type::Boolean, a, b, Value::Boolean(a<=b));

fn fn_if() -> Variable {
    function(FunctionType::curry_rr(priorities::control_flow::IF, Type::Function(FunctionType::scope(Type::Unit)),
            vec![Identifier::new(Type::Boolean, "cond")],
                                        // no fancy types yet so the if cant return anything yet
            vec![Identifier::new(Type::Function(FunctionType::scope(Type::Unit)), "then")]
        ), FunctionDefinition::Magic(Box::new(|s| {
            let cond = get_arg_boolean(s, "cond")?;
            let then = get_arg_function(s, "then")?.clone();
            if *cond {
                Ok(Value::Function(then))
            } else {
                Ok(Value::Unit) // unit is identical to empty scope
            }
    })))
}

fn fn_let() -> Variable {
    function(FunctionType::new_r(Priority::new(priorities::LET),
            vec![Identifier::new(Type::Identifier, "id_name")], Type::Identifier,
        ),
        FunctionDefinition::Magic(Box::new(|s| {
            let id_name = get_arg_identifier(s, "id_name")?.clone();
            s.declare_variable(&id_name);
            Ok(Value::Identifier(id_name))
    })))
}

fn fn_set() -> Variable {
    function(FunctionType::curry_lr(priorities::SET, Type::Unit,
            vec![Identifier::new(Type::Identifier, "name")],
            vec![Identifier::new(Type::Int, "value")]
        ), FunctionDefinition::Magic(Box::new(|s| {
            let name = get_arg_identifier(s, "name")?.clone(); // its disjoint! grr
            let value = get_arg(s, "value")?.get().clone(); // should this be cloned?
            s.set_variable(&name, value)?;
            Ok(Value::Unit)
    })))
}

fn fn_print() -> Variable {
    function(FunctionType::new_r(Priority::new(priorities::FN),
            vec![Identifier::new(Type::String, "value")], Type::Unit,
        ),
        FunctionDefinition::Magic(Box::new(|s| {
            let value = get_arg_string(s, "value")?;
            println!("{value}");
            Ok(Value::Unit)
    })))
}


