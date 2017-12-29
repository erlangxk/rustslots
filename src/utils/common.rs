use std::ops::{Add, Deref};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Idx(pub usize);
pub type Matrix = Vec<Vec<Idx>>;

impl Deref for Idx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl Add<u8> for Idx {
    type Output = Idx;

    fn add(self, other: u8) -> Idx {
        Idx(self.0 + (other as usize))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

pub type Line = Vec<Coord>;
pub type MultiLines = Vec<Vec<Coord>>;

pub struct ReelMeta {
    pub length: u8,
    pub total: usize,
}

impl ReelMeta {
    pub fn new(length: u8, total: usize) -> ReelMeta {
        ReelMeta { length, total }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Symbol(pub u8);
pub type Reel = Vec<Symbol>;
pub type ReelStrips = Vec<Vec<Symbol>>;
