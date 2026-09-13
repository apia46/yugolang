use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Priority {
    value: PriorityLayer,
    next: Option<Box<Priority>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)] // allegedly bad practice
enum PriorityLayer { NegativeInfinity, Finite(i64), Infinity }

impl Priority {
    pub fn new(value:i64) -> Self { Self { value: PriorityLayer::Finite(value), next: None } }
    pub fn minus_inf()    -> Self { Self { value: PriorityLayer::NegativeInfinity, next: None } }
    pub fn inf()          -> Self { Self { value: PriorityLayer::Infinity, next: None } }
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

