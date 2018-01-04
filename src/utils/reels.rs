use rand::{thread_rng, Rng};
use std::collections::HashMap;
use super::common::{Idx, Matrix, ReelMeta, ReelStrips, Symbol, Wheel};

fn ring(max: Idx, start: Idx, len: u8) -> Vec<Idx> {
    let last = start + len;
    let mut result: Vec<Idx> = Vec::new();
    for i in start.0..last.0 {
        result.push(Idx(i % max.0));
    }
    result
}

fn matrix<F>(reel_metas: &[ReelMeta], mut rng: F) -> Matrix
where
    F: FnMut(Idx) -> Idx,
{
    let mut result = Vec::new();
    for r in reel_metas {
        let max = Idx(r.total());
        result.push(ring(max, rng(max), r.length()));
    }
    result
}

fn rng(max: Idx) -> Idx {
    let mut rng = thread_rng();
    Idx(rng.gen_range(0, max.0))
}

pub fn random_matrix(reels_metas: &[ReelMeta]) -> Matrix {
    matrix(reels_metas, rng)
}

fn line_pick(line: &Vec<Idx>, reel: &Vec<Symbol>) -> Vec<Symbol> {
    line.iter().map(|&i| reel[*i]).collect()
}

fn line_replace(
    line: &Vec<Idx>,
    reel: &Vec<Symbol>,
    replace_table: &HashMap<Symbol, Symbol>,
) -> Vec<Symbol> {
    line.iter()
        .map(|&i| {
            let s = reel[*i];
            match replace_table.get(&s) {
                Some(&r) => r,
                None => s,
            }
        })
        .collect()
}

pub fn crop<F>(reel_strips: &ReelStrips, matrix: &Matrix, line_crop: F) -> Wheel
where
    F: Fn(&Vec<Idx>, &Vec<Symbol>) -> Vec<Symbol>,
{
    Wheel(
        matrix
            .iter()
            .zip(reel_strips.iter())
            .map(|(m, r)| line_crop(m, r))
            .collect(),
    )
}

pub fn random_spin(reels_metas: &[ReelMeta], reel_strips: &ReelStrips) -> Wheel {
    crop(reel_strips, &random_matrix(reels_metas), line_pick)
}

pub fn random_spin_replace(
    reels_metas: &[ReelMeta],
    reel_strips: &ReelStrips,
    replace_table: &HashMap<Symbol, Symbol>,
) -> Wheel {
    crop(reel_strips, &random_matrix(reels_metas), |line, reel| {
        line_replace(line, reel, replace_table)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::common::Symbol as S;
    #[test]
    fn test_ring() {
        assert_eq!(
            vec![Idx(12), Idx(13), Idx(14), Idx(0), Idx(1), Idx(2)],
            ring(Idx(15), Idx(12), 6)
        );
        assert_eq!(
            vec![Idx(12), Idx(13), Idx(14), Idx(0), Idx(1)],
            ring(Idx(15), Idx(12), 5)
        );
        assert_eq!(
            vec![Idx(0), Idx(1), Idx(2), Idx(0), Idx(1), Idx(2), Idx(0)],
            ring(Idx(3), Idx(0), 7)
        );
    }

    #[test]
    fn test_matrix() {
        let meta = vec![ReelMeta(3, 33), ReelMeta(2, 40)];
        let mut start = Idx(3);
        let rng2 = |_: Idx| {
            start = start + 1;
            start
        };
        let result = matrix(&meta, rng2);
        assert_eq!(
            result,
            vec![vec![Idx(4), Idx(5), Idx(6)], vec![Idx(5), Idx(6)]]
        );
    }

    #[test]
    fn test_crop_noop() {
        let matrix = vec![
            vec![Idx(1), Idx(3), Idx(5), Idx(2)],
            vec![Idx(7), Idx(8), Idx(9), Idx(0)],
        ];
        let reelstrips = ReelStrips(vec![
            vec![S(9), S(11), S(2), S(33), S(24), S(5)],
            vec![S(10), S(1), S(2), S(3), S(4), S(5), S(6), S(7), S(8), S(9)],
        ]);
        let result = vec![
            vec![S(11), S(33), S(5), S(2)],
            vec![S(7), S(8), S(9), S(10)],
        ];
        assert_eq!(result, crop(&reelstrips, &matrix, line_pick).0);
        assert_eq!(
            result,
            crop(&reelstrips, &matrix, |line, reel| {
                line_replace(line, reel, &HashMap::new())
            }).0
        );
    }

    #[test]
    fn test_crop_11to99() {
        let matrix = vec![
            vec![Idx(1), Idx(3), Idx(5), Idx(2)],
            vec![Idx(7), Idx(8), Idx(9), Idx(0)],
        ];
        let reelstrips = ReelStrips(vec![
            vec![S(9), S(11), S(2), S(33), S(24), S(5)],
            vec![S(10), S(1), S(2), S(3), S(4), S(5), S(6), S(7), S(8), S(9)],
        ]);
        let result = vec![
            vec![S(99), S(33), S(5), S(2)],
            vec![S(7), S(8), S(9), S(10)],
        ];
        assert_eq!(
            result,
            crop(&reelstrips, &matrix, |line, reel| {
                line_replace(line, reel, &hashmap!(S(11)=> S(99)))
            }).0
        );
    }
}
