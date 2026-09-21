use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Priority { // why is this a linked list?
    value: PriorityLayer,
    next: Option<Box<Priority>>,
}

#[derive(Debug, Clone)] 
pub enum PriorityLayer { NegativeInfinity, Finite(i64), Infinity }

impl Priority {
    pub fn new(value:i64) -> Self { Self { value: PriorityLayer::Finite(value), next: None } }
    pub fn minus_inf()    -> Self { Self { value: PriorityLayer::NegativeInfinity, next: None } }
    pub fn inf()          -> Self { Self { value: PriorityLayer::Infinity, next: None } }
    pub fn first(&self)   -> &PriorityLayer {&self.value}
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => {
                let a = match self.next { Some(ref a) => a.as_ref(), None => &Priority::new(0) };
                let b = match other.next { Some(ref b) => b.as_ref(), None => &Priority::new(0) };
                a.cmp(b)
            },
            different => different,
        }
    }
}
impl PartialOrd for Priority{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Priority{}
impl PartialEq for Priority{
    fn eq(&self, other: &Self) -> bool{
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for PriorityLayer{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::NegativeInfinity, Self::NegativeInfinity) | (Self::Infinity, Self::Infinity) => Ordering::Equal,
            (Self::NegativeInfinity, _) | (_, Self::Infinity) => Ordering::Less,
            (Self::Infinity, _) | (_, Self::NegativeInfinity) => Ordering::Greater,
            (Self::Finite(a), Self::Finite(b)) => a.cmp(b),
        }
    }
}
impl PartialOrd for PriorityLayer{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for PriorityLayer{}
impl PartialEq for PriorityLayer{
    fn eq(&self, other: &Self) -> bool{
        self.cmp(other) == Ordering::Equal
    }
}

impl std::fmt::Display for PriorityLayer{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self{
            Self::Infinity => write!(f, "∞"),
            Self::NegativeInfinity => write!(f, "-∞"),
            Self::Finite(num) => write!(f, "{num}"),
        }
    }
}