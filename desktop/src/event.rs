use std::{any::TypeId, fmt::Debug};

use crate::components::Component;

/// TODO: Eventually change this to not rely on [`TypeId`]
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EventId(TypeId);
impl Debug for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl EventId {
    /// TODO: Eventually change this to not rely on [`TypeId`]
    pub fn of<C: Component + 'static>() -> Self {
        Self(TypeId::of::<C>())
    }
}

pub trait Event {}

#[derive(Debug)]
pub struct Update;
impl Event for Update {}
