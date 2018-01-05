use super::common::{Coord, MultiLines, ReelMeta, ReelStrips, Symbol, Wheel};
use std::ops::Deref;

#[derive(Debug)]
pub struct LinesResult(pub Vec<Vec<Symbol>>);

impl Deref for LinesResult {
    type Target = Vec<Vec<Symbol>>;

    fn deref(&self) -> &Vec<Vec<Symbol>> {
        &self.0
    }
}

pub fn reel_metas_with_same_len(len: u8, reels: &ReelStrips) -> Vec<ReelMeta> {
    reels.iter().map(|r| ReelMeta(len, r.len())).collect()
}

pub fn reel_metas_with_diff_len(lens: &[u8], reels: &ReelStrips) -> Vec<ReelMeta> {
    assert_eq!(lens.len(), reels.len());
    reels
        .iter()
        .zip(lens)
        .map(|(r, l)| ReelMeta(*l, r.len()))
        .collect()
}

pub fn result_lines(lines: &MultiLines, reels: &Wheel) -> LinesResult {
    let mut result = Vec::with_capacity(lines.len());
    for line in lines {
        let mut lr = Vec::with_capacity(line.len());
        for c in line {
            lr.push(reels[c.0][c.1]);
        }
        result.push(lr);
    }
    LinesResult(result)
}

pub fn lines_result(lines: &[&[Coord]], reels: &Wheel) -> LinesResult {
    let mut result = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        let mut lr = Vec::with_capacity(line.len());
        for c in line.iter() {
            lr.push(reels[c.0][c.1]);
        }
        result.push(lr);
    }
    LinesResult(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::common::{Coord, Symbol as S};
    fn lines() -> Vec<Vec<Coord>> {
        vec![vec![(3, 0), (4, 0), (5, 0)], vec![(2, 0), (4, 0), (6, 0)]]
    }

    fn wheel() -> Wheel {
        Wheel(vec![
            vec![S(0)],
            vec![S(1)],
            vec![S(4)],
            vec![S(7)],
            vec![S(8)],
            vec![S(9)],
            vec![S(0)],
            vec![S(3)],
            vec![S(4)],
        ])
    }

    #[test]
    fn test_result_lines() {
        use utils::common::Symbol as S;
        let r = result_lines(&lines(), &wheel());
        assert_eq!(r.0, vec![vec![S(7), S(8), S(9)], vec![S(4), S(8), S(0)]]);
    }

    #[test]
    fn test_reel_metas() {
        let reelstrips = ReelStrips(vec![vec![S(0), S(1), S(2)], vec![S(3), S(4), S(5), S(6)]]);
        let r = reel_metas_with_same_len(2, &reelstrips);
        assert_eq!(r, vec![ReelMeta(2, 3), ReelMeta(2, 4)]);

        let lens = [1, 3];
        let r = reel_metas_with_diff_len(&lens, &reelstrips);
        assert_eq!(r, vec![ReelMeta(1, 3), ReelMeta(3, 4)]);
    }
}
