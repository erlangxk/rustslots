use rand::{thread_rng, Rng};

pub type Index = u16;
pub type Matrix = Vec<Vec<Index>>;

pub type Symbol = u8;
pub type ReelStrips = Vec<Vec<Symbol>>;

pub fn ring(max: Index, start: Index, len: u16) -> Vec<Index> {
    let last = start + len;
    let mut result: Vec<Index> = Vec::new();
    for i in start..last {
        result.push(i % max);
    }
    result
}

pub struct ReelMeta {
    length: u16,
    total: u16,
}

impl ReelMeta {
    pub fn new(length:u16, total:u16)-> ReelMeta {
        ReelMeta {length, total}
    }
}

pub fn matrix<F>(reels: &[ReelMeta], rng: F) -> Matrix
where
    F: Fn(Index) -> Index,
{
    let mut result = Vec::new();
    for r in reels {
        result.push(ring(r.total, rng(r.total), r.length));
    }
    result
}

#[inline(always)]
fn line_crop(line: &Vec<Index>, reel: &Vec<Symbol>) -> Vec<Symbol> {
    line.iter().map(|i| reel[*i as usize]).collect()
}

pub fn crop(reel_strips: &ReelStrips, matrix: &Matrix) -> Vec<Vec<Symbol>> {
    matrix
        .iter()
        .zip(reel_strips)
        .map(|(m, r)| line_crop(m, r))
        .collect()
}

pub fn rng(max: Index) -> Index {
    let mut rng = thread_rng();
    rng.gen_range(0, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ring() {
        assert_eq!(vec![12, 13, 14, 0, 1, 2], ring(15, 12, 6));
        assert_eq!(vec![12, 13, 14, 0, 1], ring(15, 12, 5));
        assert_eq!(vec![0, 1, 2, 0, 1, 2, 0], ring(3, 0, 7));
    }

    #[test]
    fn test_matrix() {
        let meta = [ReelMeta::new(3, 33), ReelMeta::new(2,40),];
        static mut START: u16 = 3;
        fn rng2(_x: u16) -> u16 {
            unsafe {
                START = START + 1;
                START
            }
        };
        let result = matrix(&meta, rng2);
        assert_eq!(result, vec![vec![4, 5, 6], vec![5, 6]]);
    }

    #[test]
    fn test_crop(){
        let matrix = vec![vec![1,3,5,2], vec![7,8,9,0]];
        let reel = vec![vec![9,11,2,33,24,5], vec![10,1,2,3,4,5,6,7,8,9]];
        let result = vec![vec![11,33,5,2], vec![7,8,9,10]];
        assert_eq!(result, crop(&reel, &matrix));
    }
}
