use std::{error::Error, fmt::Display};
use yugolang_parser::Scope;
use crate::typing::Type;

pub fn return_type_of(scope: &Scope) -> Result<Type,TypeError> {

   todo!()
}

#[derive(Debug)]
pub enum TypeError{
   MissingVariable{name: String},
   MismatchedTypes{expected: Type, got:Type}
}

impl Display for TypeError{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self{
         Self::MissingVariable{name} => write!(f, "variable {name} not found"),
         Self::MismatchedTypes{expected, got} => write!(f, "expected type {expected}, got {got}"), 
      }
   }
}
impl Error for TypeError{}