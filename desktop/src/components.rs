use crate::event::Event;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ComponentId(TypeId);
impl Debug for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ComponentId {
    /// TODO: Eventually change this to not rely on [`TypeId`]
    pub fn of<C: Component + 'static>() -> Self {
        Self(TypeId::of::<C>())
    }
}

pub trait Component {
    type Event: Event;
    fn update(&mut self, event: &Self::Event);
}

pub trait RawComponent {
    unsafe fn update_unchecked(&mut self, event: &dyn Any);
}
