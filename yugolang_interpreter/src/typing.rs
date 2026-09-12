use std::cmp::Ordering;
use yugolang_parser::Scope;

#[derive(Debug)]
pub enum Value {
    Unit,
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Function(Function),
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

pub type MagicFunction = Box<dyn FnMut(&mut super::state::State) -> Result<Value, super::Error>>;

pub enum FunctionDefinition {
    Scope(Scope),
    Magic(MagicFunction),
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
    input: Box<Identifier>,
    output: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction { Right, Left }

#[derive(Debug, Clone)]
pub struct Priority(Vec<PriorityLayer>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)] // allegedly bad practice
enum PriorityLayer { NegativeInfinity, Finite(i64), Infinity }

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
    pub fn new_r(priority:Priority, input:Identifier, output:Type) -> Self {
        Self {
            preferred_direction: Direction::Right,
            priority, input: Box::new(input), output: Box::new(output),
        }
    }

    pub fn new_l(priority:Priority, input:Identifier, output:Type) -> Self {
        Self {
            preferred_direction: Direction::Left,
            priority, input: Box::new(input), output: Box::new(output),
        }
    }

    /// returns a function with the left priority + 1 so that it always triggers next
    pub fn curry_lr(priority:i64, output:Type, left:Identifier, right:Identifier) -> FunctionType {
        FunctionType::new_l(Priority::new(priority), left, Type::Function(FunctionType::new_r(Priority::new(priority+1), right, output)))
    }
}

impl Priority {
    pub fn new(value:i64) -> Self { Priority(vec![PriorityLayer::Finite(value)]) }
    pub fn minus_inf()    -> Self { Priority(vec![PriorityLayer::NegativeInfinity]) }
    pub fn inf()          -> Self { Priority(vec![PriorityLayer::Infinity]) }
}

impl Ord for PriorityLayer{
    fn cmp(&self, other: &Self) -> Ordering{
    match self{
            Self::NegativeInfinity => {
                if let Self::NegativeInfinity = other{
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            },
            Self::Finite(number) => {
                match other{
                    Self::Infinity => Ordering::Less,
                    Self::NegativeInfinity => Ordering::Greater,
                    Self::Finite(other_number) => number.cmp(other_number),
                }
            },
            Self::Infinity => {
                if let Self::Infinity = other{
                    Ordering::Equal
                }else{
                    Ordering::Greater
                }
            },
        }
    }
}

impl std::fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = f.debug_tuple("FunctionDefinition");
        match self {
            FunctionDefinition::Scope(scope) => t.field(scope),
            FunctionDefinition::Magic(_) => t.field(&"<Magic>"),
        }.finish()
    }
}

