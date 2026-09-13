use yugolang_parser::{Scope, Statement, Expression, Literal};
use state::State;

use crate::typing::Value;

mod state;
mod typing;
mod global;
mod priorities;

pub enum Error {
    TypeError,
    MissingVariableError,
    DivideByZeroError,
}

pub fn interpret(ast:Scope) {
    let mut state = State::new();
    interpret_scope(ast, &mut state);
}

fn interpret_scope(scope:Scope, state:&mut State) -> Option<Value> {
    for statement in scope.statements {
        interpret_statement(statement, state);
    }
    scope.result.and_then(|s| match interpret_statement(s, state) {
        Value::Unit => None,
        value => Some(value),
    })
}

fn interpret_statement(statement:Statement, state:&mut State) -> Value {
    todo!()
}

