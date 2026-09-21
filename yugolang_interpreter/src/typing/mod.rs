use std::{fmt::Display, rc::Rc};
use yugolang_parser::Scope;

pub use priority::Priority;

mod priority;
mod static_analysis;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Function(Rc<Function>),
    Identifier(String),
}

#[derive(Debug)]
pub struct Function {
    type_info: FunctionType, // perhaps wrap this in a Rc because it gets cloned ?often?
    definition: FunctionDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    id_type: Type,
    name: String,
}

pub type MagicFunction = fn(&mut super::state::State) -> Result<Value, super::Error>;

pub enum FunctionDefinition {
    Scope(Scope),
    Magic(MagicFunction),
    MagicCurried(Box<FunctionDefinition>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type { // this too is a linked list. a trie even
    Unit,
    String,
    Int,
    Float,
    Boolean,
    Function(FunctionType),
    Identifier,
}

impl Display for Type{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Unit => write!(f, "()"),
            Self::String => write!(f, "string"),
            Self::Int => write!(f, "integer"),
            Self::Float => write!(f, "float"),
            Self::Boolean => write!(f, "bool"),
            Self::Function(fun) => write!(f, "{fun}"),
            Self::Identifier => write!(f, "identifier"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // copy since it only has two states
pub enum Direction { Right, Left }

impl Direction {
    pub fn to_offset(&self) -> usize {
        match self {
            Self::Right => 1,
            Self::Left => usize::MAX, // meant to be overflowingly added
        }
    }

    pub fn other_way(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

impl Display for Direction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Left => "L",
            Self::Right => "R", 
        })
    }
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Unit => Type::Unit,
            Self::String(_) => Type::String,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Boolean(_) => Type::Boolean,
            Self::Function(f) => Type::Function(f.type_info.clone()),
            Self::Identifier(_) => Type::Identifier
        }
    }
}

impl From<&Value> for Type{
    fn from(value: &Value) -> Type{
        value.get_type()
    }
}

impl Function {
    pub fn new(type_info:FunctionType, definition:FunctionDefinition) -> Self {
        Self { type_info, definition }
    }

    pub fn get_type(&self) -> &FunctionType { &self.type_info }
}

impl Identifier {
    pub fn new(id_type:Type, name:impl Into<String>) -> Self {
        Self { id_type, name: name.into() }
    }

    pub fn get_type(&self) -> &Type {
        &self.id_type
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    preferred_direction: Direction,
    priority: Priority,
    input: Vec<Identifier>,
    output: Box<Type>,
}

impl FunctionType {
    pub fn scope(output:Type) -> Self {
        Self {
            preferred_direction: Direction::Right,
            priority:Priority::minus_inf(), input: vec![], output: Box::new(output)
        }
    }

    pub fn new(preferred_direction:Direction, priority:Priority, input:Vec<Identifier>, output:Type) -> Self {
        Self {
            preferred_direction, priority, input, output: Box::new(output),
        }
    }

    pub fn new_r(priority:Priority, input:Vec<Identifier>, output:Type) -> Self {
        Self {
            preferred_direction: Direction::Right,
            priority, input, output: Box::new(output),
        }
    }

    pub fn new_l(priority:Priority, input:Vec<Identifier>, output:Type) -> Self {
        Self {
            preferred_direction: Direction::Left,
            priority, input, output: Box::new(output),
        }
    }

    pub fn get_priority(&self) -> &Priority { &self.priority }

    pub fn get_input(&self) -> &[Identifier] { &self.input }

    pub fn get_direction(&self) -> Direction {self.preferred_direction}
}

impl Display for FunctionType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "func [{}, {}] ({:?}) -> {}", self.preferred_direction, self.priority.first(), self.input, self.output)
    }
}

impl std::fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = f.debug_tuple("FunctionDefinition");
        match self {
            FunctionDefinition::Scope(scope) => t.field(scope),
            FunctionDefinition::Magic(_) => t.field(&"<Magic>"),
            FunctionDefinition::MagicCurried(_) => t.field(&"<MagicCurried>"),
        }.finish()
    }
}

