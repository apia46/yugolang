use std::collections::HashMap;
use nonempty::{NonEmpty, nonempty};
use crate::Error;
use crate::typing::Value;
use crate::global::global_frame;

pub struct State {
    stack:NonEmpty<Frame>
}

impl State {
    pub fn new() -> Self {
        let global_frame = global_frame();
        Self {
            stack: nonempty![global_frame],
        }
    }

    pub fn declare_variable(&mut self, name:&str) {
        self.stack.last_mut().declare_variable(name);
    }

    pub fn get_variable(&self, name:&str) -> Option<&Variable> {
        for frame in self.stack.iter().rev() {
            if let Some(v) = frame.variables.get(name) {return Some(v)}
        }
        return None
    }

    pub fn set_variable(&mut self, name:&str, to:Value) -> Result<(), Error> {
        for frame in self.stack.iter_mut().rev() {
            if let Some(v) = frame.variables.get_mut(name) {
                v.set(to);
                return Ok(());
            }
        }
        Err(Error::MissingVariableError)
    }
}

pub struct Frame {
    variables:HashMap<String, Variable>,
}

impl Frame {
    pub fn new(variables:HashMap<String, Variable>) -> Self {
        Self { variables }
    }

    pub fn declare_variable(&mut self, name:&str) { // for now, bare minimum
        self.variables.insert(name.into(), Variable::new(Value::Int(0)));
    }

}

pub struct Variable {
    value:Value,
}

impl Variable {
    pub fn new(value:Value) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &Value { &self.value }
    pub fn set(&mut self, value:Value) { self.value = value }
}

