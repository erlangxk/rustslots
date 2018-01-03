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

pub type Coord = (usize, usize);
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


pub trait Spin {
    fn spin(&self, line_bet: f64) -> (f64, f64);
}