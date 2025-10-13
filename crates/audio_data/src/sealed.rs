use crate::content::{Audio, Script};

pub trait Sealed {}
impl Sealed for Audio {}
impl Sealed for Script {}
