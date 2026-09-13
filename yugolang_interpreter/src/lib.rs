use yugolang_parser::{Scope, Statement, Expression, Literal};
use state::State;
use typing::{Function, Value, Priority};

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
    loop {
        let Some(function_index) = function_to_evaluate(&expressions, &state)? else {
            todo!()
        };
    }
}

fn function_to_evaluate(expressions:&Vec<Expression>, state:&State) -> Result<Option<usize>, Error> {
    let mut best_value = None;
    let mut best = None;
    let mut index:usize = 0;
    for expression in expressions {
        let Some(function) = interpret_as_function(expression, state)? else { continue };
        let priority = function.get_priority();
        if best_value.is_none_or(|ref best| (&priority) >= best) {
            best_value = Some(priority);
            best = Some(index);
        }
        index += 1;
    }
    Ok(best)
}

enum FunctionInterpretation<'a> {
    EmptyScope,
    Function(&'a Function),
    Scope(&'a Scope),
    ClosureArgs(&'a Scope),
}

fn interpret_as_function<'a>(expression:&'a Expression, state:&'a State) -> Result<Option<FunctionInterpretation<'a>>, Error> {
    Ok(Some(match expression {
        Expression::Identifier(s) => {
            let Some(variable) = state.get_variable(s) else { return Err(Error::MissingVariableError) };
            todo!()
        },
        Expression::Scope(s) => FunctionInterpretation::Scope(s.as_ref()),
        Expression::ClosureArgs(s) => FunctionInterpretation::ClosureArgs(s.as_ref()),
        _ => return Ok(None),
    }))
}

impl FunctionInterpretation<'_> {
    fn get_priority(&self) -> Priority {
        todo!()
    }
}

