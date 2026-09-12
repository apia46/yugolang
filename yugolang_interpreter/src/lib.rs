use yugolang_parser::{Scope, Statement, Expression, Literal};
use state::State;

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
    let state = State::new();
    interpret_scope(ast, state);
}

fn interpret_scope(scope:Scope, state:State) -> State {
    state
}

