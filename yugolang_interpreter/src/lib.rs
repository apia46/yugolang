use std::rc::Rc;
use yugolang_parser::{Scope, Statement, Expression, Literal};
use state::State;
use typing::{Function, Value, Priority, Direction, Identifier, Type};

use crate::EvaluateDetails::NoArguments;

mod state;
mod typing;
mod global;
mod priorities;

#[derive(Debug)]
pub enum Error {
    TypeError,
    MissingVariableError,
    DivideByZeroError,
}

pub fn interpret(ast:Scope) -> Result<Option<Value>, Error> {
    let mut state = State::new();
    Ok(interpret_scope(ast, &mut state)?)
}

fn interpret_scope(scope:Scope, state:&mut State) -> Result<Option<Value>, Error> {
    for statement in scope.statements {
        eprintln!("Interpreting statement {statement:?}");
        interpret_statement(statement, state)?;
    }
    Ok(match scope.result {
        Some(s) => {
            eprintln!("Interpreting result {s:?}");
            match interpret_statement(s, state)? {
                Value::Unit => None,
                value => Some(value),
            }
        },
        None => None,
    })
}
fn interpret_statement(Statement(expressions):Statement, state:&mut State) -> Result<Value, Error> {
    // let mut expressions = expressions.clone(); // ughhhh
    //                                                             // i think we should work with references
    //                                                             // and have the inner scope be in an            the what? -k
    //                                                             // Rc instead of a block but im not sure
    //                                                             // we need a shallowly mutable copy here
    //                                                             // for the in-place operations
    let mut expressions: Vec<_> = expressions.iter().collect();
    loop {
        eprintln!("\tCurrent state of ment: {expressions:?}");
        let Some((mut function_index, evaluate_details)) = function_to_evaluate(&expressions, &state)? else {
            todo!("Found no function to evaluate");
        };
        eprintln!("\tEvaluating {:?}", expressions[function_index]);
        let result = match evaluate_details {
            EvaluateDetails::NoArguments(function) => {
                eprintln!("\t\tIt takes no arguments");
                expressions.remove(function_index);
                function.evaluate(Value::Unit)
            },
            EvaluateDetails::Arguments(function, direction) => {
                expressions.remove(function_index);
                if matches!(direction, Direction::Left) { function_index -= 1 };
                let argument_type = function.get_inputs().get(0).expect("Arguments function with no arguments").get_type();
                eprintln!("\t\tIt takes an argument of type {argument_type:?}: {:?}", expressions[function_index]);
                let argument = interpret_as(&expressions.remove(function_index), argument_type, state)?;
                function.evaluate(argument)
            }
        };
    }
}

fn function_to_evaluate<'a, 's>(expressions: &[&'a Expression], state:&'s State) -> Result<Option<(usize, EvaluateDetails<'a>)>, Error> {
    let mut best_value = None;
    let mut best = None;
    let mut index:usize = 0;
    eprintln!("\t\tfinding function to evaluate in {expressions:?}");
    for expression in expressions {
        let Some(function) = interpret_as_function(expression, state)? else {
            eprintln!("\t\t\t...it is not a function"); 
            continue 
        };
        let priority = function.get_priority();
        eprintln!("\t\t\tits priority is {priority:?}");
        // if theres a tie, we want the rightmost leftwards function, or if not then the leftmost rightwards function.
        // therefore, we only take a rightwards function if its a new best and therefore the leftest
        // but we always take a leftwards function, since itll be more rightwards than whatever we had before
        if best_value.as_ref().is_none_or(|best| &priority > best
            || (&priority == best && matches!(function.get_direction(), Direction::Left))) {
            if let Some(details) = get_evaluate_details(expressions, index, function, state)? {
                eprintln!("\t\t\t\twhich is higher than the previous best find");
                best_value = Some(priority);
                best = Some((index, details));
            }
        }
        index += 1;
    }
    Ok(best)
}

enum EvaluateDetails<'a> {
    NoArguments(FunctionInterpretation<'a>),
    Arguments(FunctionInterpretation<'a>, Direction),
}

fn get_evaluate_details<'a>(expressions: &[&Expression], index:usize, function:FunctionInterpretation<'a>, state:&State) -> Result<Option<EvaluateDetails<'a>>, Error> {
    let mut direction = function.get_direction();
    let input_types = function.get_inputs();
    if input_types.is_empty() { return Ok(Some(EvaluateDetails::NoArguments(function))) }
    let input_type = input_types.get(0).unwrap();
    assert!(input_types.len() == 1); // no arrays yet
    for _ in 0..2 {
        if let Some(expression) = expressions.get(direction.to_offset().overflowing_add(index).0) {
            if can_interpret_as(expression, input_type.get_type(), state)? {
                return Ok(Some(EvaluateDetails::Arguments(function, direction)));
            }
        }
        direction = direction.other_way();
    }
    Ok(None)
}

// what does this signature mean?
fn interpret_as_function<'a, 's>(expression:&'a Expression, state:&'s State) -> Result<Option<FunctionInterpretation<'a>>, Error> {
    eprintln!("\t\t\tconsidering {expression:?}");
    Ok(Some(match expression {
        Expression::Identifier(s) => {
            let Some(variable) = state.get_variable(s) else {
                eprintln!{"\t\t\t\tthere is no such variable"};
                // return Err(Error::MissingVariableError) 
                return Ok(None)
            };
            match variable.get() {
                Value::Unit => FunctionInterpretation::EmptyScope,
                Value::Function(f) => FunctionInterpretation::Function(f.clone()),
                _ => return Ok(None),
            }
        },
        Expression::Scope(s) => FunctionInterpretation::Scope(s.as_ref()),
        Expression::ClosureArgs(s) => FunctionInterpretation::ClosureArgs(s.as_ref()),
        _ => return Ok(None),
    }))
}

fn can_interpret_as(expression:&Expression, interpret_type:&Type, state:&State) -> Result<bool, Error> {
    use Expression as E;
    match expression{
        E::Literal(literal) => {
            use Literal as L;
            Ok(match literal{
                L::Int(_) => matches!(interpret_type, Type::Int),
                L::Float(_) => matches!(interpret_type, Type::Float),
                L::String(_) => matches!(interpret_type, Type::String),
            })
        }
        E::Identifier(ident) =>{
            Ok(state.get_variable(ident).and_then(|var| (var.get().get_type() == *interpret_type).then_some(())).is_some())
        }
        E::Scope(scp) => {
            let scp = scp.as_ref();
            todo!("ill finish this tomorrow")
        }
        E::ClosureArgs(_) => todo!("ClosureArgs")
    }
}

fn interpret_as(expression:&Expression, interpret_type:&Type, state:&State) -> Result<Value, Error> {
    todo!()
}

enum FunctionInterpretation<'a> {
    Function(Rc<Function>),
    EmptyScope,
    Scope(&'a Scope),
    ClosureArgs(&'a Scope),
}

impl FunctionInterpretation<'_> {
    fn get_priority(&self) -> Priority {
        match self{
            Self::EmptyScope | Self::Scope(_) => Priority::minus_inf(),
            Self::Function(fun) => fun.get_type().get_priority().clone(), // this clone sucks
            Self::ClosureArgs(_) => todo!("i have no idea what these are or how they're supposed to behave")
        }
    }
    
    fn get_direction(&self) -> Direction {
        match self{
            Self::EmptyScope | Self::Scope(_) => Direction::Right,
            Self::Function(fun) => fun.get_type().get_direction(),
            Self::ClosureArgs(_) => todo!("ClosureArgs")
        }
    }

    fn get_input(&self) -> Type {
        todo!()
    }

    fn evaluate(self, argument:Value) -> Value {
        todo!()
    }
    fn get_inputs(&self) -> &[Identifier]{
        match self{
            Self::EmptyScope | Self::Scope(_) => &[],
            Self::Function(fun) => fun.get_type().get_input(),
            Self::ClosureArgs(_) => todo!("idk what a ClosureArgs is")
        }
    }
}

