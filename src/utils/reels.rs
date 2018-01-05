use rand::{thread_rng, Rng};
use std::collections::HashMap;
use super::common::{Matrix, ReelMeta, ReelStrips, Symbol, Wheel};

type ReplacementTable = HashMap<Symbol, Symbol>;
type Line = Vec<usize>;
type Reel = Vec<Symbol>;

fn ring(max: usize, start: usize, len: u8) -> Vec<usize> {
    let lus = len as usize;
    let mut result: Vec<usize> = Vec::with_capacity(lus);
    for i in start..(start + lus) {
        result.push(i % max);
    }
    result
}

fn matrix<F>(reel_metas: &[ReelMeta], mut rng: F) -> Matrix
where
    F: FnMut(usize) -> usize,
{
    let mut result = Vec::with_capacity(reel_metas.len());
    for r in reel_metas {
        let max = r.total();
        result.push(ring(max, rng(max), r.length()));
    }
    result
}

fn rng(max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, max)
}

pub fn random_matrix(reels_metas: &[ReelMeta]) -> Matrix {
    matrix(reels_metas, rng)
}

fn line_pick(line: &Line, reel: &Reel) -> Vec<Symbol> {
    line.iter().map(|&i| reel[i]).collect()
}

fn line_replace(line: &Line, reel: &Reel, replace_table: &ReplacementTable) -> Vec<Symbol> {
    line.iter()
        .map(|&i| {
            let s = reel[i];
            match replace_table.get(&s) {
                Some(&r) => r,
                None => s,
            }
        })
        .collect()
}

pub fn crop<F>(reel_strips: &ReelStrips, matrix: &Matrix, line_crop: F) -> Wheel
where
    F: Fn(&Line, &Reel) -> Vec<Symbol>,
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
    replace_table: &ReplacementTable,
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
        assert_eq!(vec![12, 13, 14, 0, 1, 2], ring(15, 12, 6));
        assert_eq!(vec![12, 13, 14, 0, 1], ring(15, 12, 5));
        assert_eq!(vec![0, 1, 2, 0, 1, 2, 0], ring(3, 0, 7));
    }

    #[test]
    fn test_matrix() {
        let meta = vec![ReelMeta(3, 33), ReelMeta(2, 40)];
        let mut start = 3;
        let rng2 = |_: usize| {
            start = start + 1;
            start
        };
        let result = matrix(&meta, rng2);
        assert_eq!(result, vec![vec![4, 5, 6], vec![5, 6]]);
    }

    #[test]
    fn test_crop_noop() {
        let matrix = vec![vec![1, 3, 5, 2], vec![7, 8, 9, 0]];
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
        let matrix = vec![vec![1, 3, 5, 2], vec![7, 8, 9, 0]];
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
