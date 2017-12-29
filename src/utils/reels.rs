use rand::{thread_rng, Rng};
use std::ops::{Add, Deref};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Idx(u16);

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

///pub type Idx = u16;
pub type Matrix = Vec<Vec<Idx>>;

pub type Symbol = u8;
pub type ReelStrips = Vec<Vec<Symbol>>;

pub fn ring(max: Idx, start: Idx, len: u8) -> Vec<Idx> {
    let last = start + len;
    let mut result: Vec<Idx> = Vec::new();
    for i in start.0..last.0 {
        result.push(Idx(i % max.0));
    }
    result
}

pub struct ReelMeta {
    length: u8,
    total: u16,
}

impl ReelMeta {
    pub fn new(length: u8, total: u16) -> ReelMeta {
        ReelMeta { length, total }
    }
}

pub fn matrix<F>(reels: &[ReelMeta], rng: F) -> Matrix
where
    F: Fn(Idx) -> Idx,
{
    let mut result = Vec::new();
    for r in reels {
        let max = Idx(r.total);
        result.push(ring(max, rng(max), r.length));
    }
    result
}

#[inline(always)]
fn line_crop(line: &Vec<Idx>, reel: &Vec<Symbol>) -> Vec<Symbol> {
    line.iter().map(|i| reel[**i as usize]).collect()
}

pub fn crop(reel_strips: &ReelStrips, matrix: &Matrix) -> Vec<Vec<Symbol>> {
    matrix
        .iter()
        .zip(reel_strips)
        .map(|(m, r)| line_crop(m, r))
        .collect()
}

pub fn rng(max: Idx) -> Idx {
    let mut rng = thread_rng();
    Idx(rng.gen_range(0, max.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ring() {
        assert_eq!(vec![Idx(12), Idx(13), Idx(14), Idx(0), Idx(1), Idx(2)], ring(Idx(15), Idx(12), 6));
        assert_eq!(vec![Idx(12), Idx(13), Idx(14), Idx(0), Idx(1)], ring(Idx(15), Idx(12), 5));
        assert_eq!(vec![Idx(0), Idx(1), Idx(2), Idx(0), Idx(1), Idx(2), Idx(0)], ring(Idx(3), Idx(0), 7));
    }

    #[test]
    fn test_matrix() {
        let meta = [ReelMeta::new(3, 33), ReelMeta::new(2, 40)];
        static mut START: Idx = Idx(3);
        fn rng2(_x: Idx) -> Idx {
            unsafe {
                START = Idx(START.0 + 1);
                START
            }
        };
        let result = matrix(&meta, rng2);
        assert_eq!(result, vec![vec![Idx(4), Idx(5), Idx(6)], vec![Idx(5), Idx(6)]]);
    }

    #[test]
    fn test_crop() {
        let matrix = vec![
            vec![Idx(1), Idx(3), Idx(5), Idx(2)],
            vec![Idx(7), Idx(8), Idx(9), Idx(0)],
        ];
        let reel = vec![
            vec![9, 11, 2, 33, 24, 5],
            vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        let result = vec![vec![11, 33, 5, 2], vec![7, 8, 9, 10]];
        assert_eq!(result, crop(&reel, &matrix));
    }
}
