use super::common::{MultiLines, ReelMeta, ReelStrips, Symbol};

pub fn reel_metas_with_same_len(len: u8, reels: &ReelStrips) -> Vec<ReelMeta> {
    reels.iter().map(|r| ReelMeta(len, r.len())).collect()
}

pub fn result_lines(lines: &MultiLines, reels: &ReelStrips) -> Vec<Vec<Symbol>> {
    let mut result = Vec::new();
    for line in lines {
        let mut lr = Vec::new();
        for c in line {
            lr.push(reels[c.0][c.1]);
        }
        result.push(lr);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::common::{Coord, Symbol as S};
    fn lines() -> Vec<Vec<Coord>> {
        vec![vec![(3, 0), (4, 0), (5, 0)], vec![(2, 0), (4, 0), (6, 0)]]
    }

    fn reels() -> Vec<Vec<S>> {
        vec![
            vec![S(0)],
            vec![S(1)],
            vec![S(4)],
            vec![S(7)],
            vec![S(8)],
            vec![S(9)],
            vec![S(0)],
            vec![S(3)],
            vec![S(4)],
        ]
    }

    #[test]
    fn test_result_lines() {
        use utils::common::Symbol as S;
        let r = result_lines(&lines(), &reels());
        assert_eq!(r, vec![vec![S(7), S(8), S(9)], vec![S(4), S(8), S(0)]]);
    }
}
