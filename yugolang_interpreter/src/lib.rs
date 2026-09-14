use std::rc::Rc;
use yugolang_parser::{Scope, Statement, Expression, Literal};
use state::State;
use typing::{Function, Value, Priority, Direction, Identifier, Type};

use crate::EvaluateDetails::NoArguments;

mod state;
mod typing;
mod global;
mod priorities;

pub enum Error {
    TypeError,
    MissingVariableError,
    DivideByZeroError,
}

pub fn interpret(ast:Scope) -> Result<(), Error> {
    let mut state = State::new();
    interpret_scope(ast, &mut state)?;
    Ok(())
}

fn interpret_scope(scope:Scope, state:&mut State) -> Result<Option<Value>, Error> {
    for statement in scope.statements {
        interpret_statement(statement, state)?;
    }
    Ok(match scope.result {
        Some(s) => match interpret_statement(s, state)? {
            Value::Unit => None,
            value => Some(value),
        },
        None => None,
    })
}
fn interpret_statement(Statement(expressions):Statement, state:&mut State) -> Result<Value, Error> {
    let mut expressions = expressions.clone(); // ughhhh
                                                                // i think we should work with references
                                                                // and have the inner scope be in an
                                                                // Rc instead of a block but im not sure
                                                                // we need a shallowly mutable copy here
                                                                // for the in-place operations
    loop {
        let Some((mut function_index, evaluate_details)) = function_to_evaluate(&expressions, &state)? else {
            todo!()
        };
        let result = match evaluate_details {
            EvaluateDetails::NoArguments(function) => {
                expressions.remove(function_index);
                function.evaluate(Value::Unit)
            },
            EvaluateDetails::Arguments(function, direction) => {
                expressions.remove(function_index);
                if matches!(direction, Direction::Left) { function_index -= 1 };
                let argument = interpret_as(&expressions.remove(function_index), function.get_inputs().iter().map(|i| i.get_type()).collect(), state)?;
                function.evaluate(argument)
            }
        };
    }
}

fn function_to_evaluate<'a>(expressions:&'a Vec<Expression>, state:&'a State) -> Result<Option<(usize, EvaluateDetails<'a>)>, Error> {
    let mut best_value = None;
    let mut best = None;
    let mut index:usize = 0;
    for expression in expressions {
        let Some(function) = interpret_as_function(expression, state)? else { continue };
        let priority = function.get_priority();
        // if theres a tie, we want the rightmost leftwards function, or if not then the leftmost rightwards function.
        // therefore, we only take a rightwards function if its a new best and therefore the leftest
        // but we always take a leftwards function, since itll be more rightwards than whatever we had before
        if best_value.as_ref().is_none_or(|best| &priority > best
            || (&priority == best && matches!(function.get_direction(), Direction::Left))) {
            if let Some(details) = get_evaluate_details(expressions, index, function, state)? {
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

fn get_evaluate_details<'a>(expressions:&Vec<Expression>, index:usize, function:FunctionInterpretation<'a>, state:&State) -> Result<Option<EvaluateDetails<'a>>, Error> {
    let mut direction = function.get_direction();
    let input_types = function.get_inputs();
    if input_types.is_empty() { return Ok(Some(EvaluateDetails::NoArguments(function))) }
    let input_type = input_types.get(0).unwrap();
    assert!(input_types.len() == 1); // no arrays yet
    for _ in 0..2 {
        if let Some(expression) = expressions.get(index+direction.to_offset()) {
            if can_interpret_as(expression, input_type.get_type(), state)? {
                return Ok(Some(EvaluateDetails::Arguments(function, direction)));
            }
        }
        direction = direction.other_way();
    }
    Ok(None)
}

enum FunctionInterpretation<'a> {
    Function(Rc<Function>),
    EmptyScope,
    Scope(&'a Scope),
    ClosureArgs(&'a Scope),
}

fn interpret_as_function<'a>(expression:&'a Expression, state:&'a State) -> Result<Option<FunctionInterpretation<'a>>, Error> {
    Ok(Some(match expression {
        Expression::Identifier(s) => {
            let Some(variable) = state.get_variable(s) else { return Err(Error::MissingVariableError) };
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
    todo!()
}

fn interpret_as(expression:&Expression, interpret_type:&Type, state:&State) -> Result<Value, Error> {
    todo!()
}

impl FunctionInterpretation<'_> {
    fn get_priority(&self) -> Priority {
        todo!()
    }
    
    fn get_direction(&self) -> Direction {
        todo!()
    }

    fn get_input(&self) -> Type {
        todo!()
    }

    fn evaluate(self, argument:Value) -> Value {
        todo!()
    }
}

