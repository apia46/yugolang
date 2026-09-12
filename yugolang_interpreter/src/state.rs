use std::collections::HashMap;
use super::{typing::Value, global::global_frame};

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
}

