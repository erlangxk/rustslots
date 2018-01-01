use std::ops::{Add, Deref};
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ReelMeta(pub u8, pub usize);

impl ReelMeta {
    #[inline(always)]
    pub fn length(&self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub fn total(&self) -> usize {
        self.1
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Symbol(pub u8);
pub type Reel = Vec<Symbol>;
pub type ReelStrips = Vec<Vec<Symbol>>;


pub type PayTable = HashMap<Symbol, HashMap<usize, u16>>;