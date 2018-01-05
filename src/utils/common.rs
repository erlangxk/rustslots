use std::ops::Deref;

pub type Matrix = Vec<Vec<usize>>;
pub type Coord = (usize, usize);

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

#[derive(Debug)]
pub struct ReelStrips(pub Vec<Vec<Symbol>>);

impl Deref for ReelStrips {
    type Target = Vec<Vec<Symbol>>;

    fn deref(&self) -> &Vec<Vec<Symbol>> {
        &self.0
    }
}

#[derive(Debug)]
pub struct Wheel(pub Vec<Vec<Symbol>>);
impl Deref for Wheel {
    type Target = Vec<Vec<Symbol>>;

    fn deref(&self) -> &Vec<Vec<Symbol>> {
        &self.0
    }
}

pub trait Spin {
    fn spin(&self, line_bet: f64) -> (f64, f64);
}
