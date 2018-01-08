use super::common::{Coord, Symbol, Wheel};
use std::ops::Deref;

#[derive(Debug)]
pub struct LinesResult(pub Vec<Vec<Symbol>>);

impl Deref for LinesResult {
    type Target = Vec<Vec<Symbol>>;

    fn deref(&self) -> &Vec<Vec<Symbol>> {
        &self.0
    }
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
        let lines: [&[Coord]; 2] = [&[(3, 0), (4, 0), (5, 0)], &[(2, 0), (4, 0), (6, 0)]];
        use utils::common::Symbol as S;
        let r = lines_result(&lines, &wheel());
        assert_eq!(r.0, vec![vec![S(7), S(8), S(9)], vec![S(4), S(8), S(0)]]);
    }
}
