pub type Matrix = Vec<Vec<u16>>;

pub fn ring(max: u16, start: u16, len: u16) -> Vec<u16> {
    let last = start + len;
    let mut result: Vec<u16> = Vec::new();
    for i in start..last {
        result.push(i % max);
    }
    result
}

pub struct ReelMeta {
    length: u16,
    total: u16,
}

pub fn matrix<F>(reels: &[ReelMeta], rng: F) -> Matrix
where
    F: Fn(u16) -> u16,
{
    let mut result = Vec::new();
    for r in reels {
        result.push(ring(r.total, rng(r.total), r.length));
    }
    result
}
