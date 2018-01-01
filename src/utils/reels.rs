use rand::{thread_rng, Rng};

use super::common::{Idx, Matrix, Reel, ReelMeta, ReelStrips};

pub fn ring(max: Idx, start: Idx, len: u8) -> Vec<Idx> {
    let last = start + len;
    let mut result: Vec<Idx> = Vec::new();
    for i in start.0..last.0 {
        result.push(Idx(i % max.0));
    }
    result
}

pub fn matrix<F>(reels: &[ReelMeta], mut rng: F) -> Matrix
where
    F: FnMut(Idx) -> Idx,
{
    let mut result = Vec::new();
    for r in reels {
        let max = Idx(r.total);
        result.push(ring(max, rng(max), r.length));
    }
    result
}

#[inline(always)]
fn line_crop(line: &Vec<Idx>, reel: &Reel) -> Reel {
    line.iter().map(|i| reel[**i]).collect()
}

pub fn crop(reel_strips: &ReelStrips, matrix: &Matrix) -> ReelStrips {
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
    use super::super::common::Symbol;
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
        let meta = [ReelMeta::new(3, 33), ReelMeta::new(2, 40)];
        let mut start = Idx(3);
        let rng2 = |_:Idx| {
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
    fn test_crop() {
        let matrix = vec![
            vec![Idx(1), Idx(3), Idx(5), Idx(2)],
            vec![Idx(7), Idx(8), Idx(9), Idx(0)],
        ];
        let reel = vec![
            vec![
                Symbol(9),
                Symbol(11),
                Symbol(2),
                Symbol(33),
                Symbol(24),
                Symbol(5),
            ],
            vec![
                Symbol(10),
                Symbol(1),
                Symbol(2),
                Symbol(3),
                Symbol(4),
                Symbol(5),
                Symbol(6),
                Symbol(7),
                Symbol(8),
                Symbol(9),
            ],
        ];
        let result = vec![
            vec![Symbol(11), Symbol(33), Symbol(5), Symbol(2)],
            vec![Symbol(7), Symbol(8), Symbol(9), Symbol(10)],
        ];
        assert_eq!(result, crop(&reel, &matrix));
    }
}
