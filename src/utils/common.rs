use std::ops::{Add, Deref};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Idx(pub u16);
pub type Matrix = Vec<Vec<Idx>>;

impl Deref for Idx {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl Add<u8> for Idx {
    type Output = Idx;

    fn add(self, other: u8) -> Idx {
        Idx(self.0 + (other as u16))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord(pub u16, pub u16);

pub struct ReelMeta {
    pub length: u8,
    pub total: u16,
}

impl ReelMeta {
    pub fn new(length: u8, total: u16) -> ReelMeta {
        ReelMeta { length, total }
    }
}