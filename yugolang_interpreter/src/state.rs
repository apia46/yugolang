use std::collections::HashMap;
use crate::Error;
use crate::typing::Value;
use crate::global::global_frame;

pub struct State {
    stack:Vec<Frame>
}

impl State {
    pub fn new() -> Self {
        let global_frame = global_frame();
        Self {
            stack: vec![global_frame],
        }
    }

    pub fn get_variable(&self, name:&str) -> Option<&Variable> {
        for frame in self.stack.iter().rev() {
            if let Some(v) = frame.variables.get(name) {return Some(v)}
        }
        return None
    }

    pub fn get_variable_mut(&mut self, name:&str) -> Option<&mut Variable> {
        for frame in self.stack.iter_mut().rev() {
            if let Some(v) = frame.variables.get_mut(name) {return Some(v)}
        }
        return None
    }
}

pub struct Frame {
    variables:HashMap<String, Variable>,
}

impl Frame {
    pub fn new(variables:HashMap<String, Variable>) -> Self {
        Self { variables }
    }
}

pub struct Variable {
    mutable:bool,
    value:Value,
}

impl Variable {
    pub fn new(value:Value) -> Self {
        Self { mutable: false, value, }
    }

    pub fn new_mut(value:Value) -> Self {
        Self { mutable: true, value, }
    }

    pub fn get(&self) -> &Value { &self.value }
    pub fn get_mut(&mut self) -> &mut Value { &mut self.value }
}

