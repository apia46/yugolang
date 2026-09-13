use std::rc::Rc;
use yugolang_parser::Scope;

pub use priority::Priority;

mod priority;

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
    type_info: FunctionType,
    definition: FunctionDefinition,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    String,
    Int,
    Float,
    Boolean,
    Function(FunctionType),
    Identifier,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    preferred_direction: Direction,
    priority: Priority,
    input: Vec<Identifier>,
    output: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction { Right, Left }
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
}

impl Identifier {
    pub fn new(id_type:Type, name:impl Into<String>) -> Self {
        Self { id_type, name: name.into() }
    }
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

