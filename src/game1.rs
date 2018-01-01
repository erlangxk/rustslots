use utils::common::{Coord as C, PayTable, ReelMeta as M, Symbol as S};
use utils::subst::ParseResult;

pub fn lines() -> Vec<Vec<C>> {
    vec![
        vec![C(3, 0), C(4, 0), C(5, 0)],
        vec![C(0, 0), C(1, 0), C(2, 0)],
        vec![C(6, 0), C(7, 0), C(8, 0)],
        vec![C(0, 0), C(3, 0), C(6, 0)],
        vec![C(1, 0), C(4, 0), C(7, 0)],
        vec![C(2, 0), C(5, 0), C(8, 0)],
        vec![C(0, 0), C(4, 0), C(8, 0)],
        vec![C(2, 0), C(4, 0), C(6, 0)],
    ]
}

pub fn reel_metas() -> Vec<M> {
    vec![
        M(1, 33),
        M(1, 33),
        M(1, 33),
        M(1, 40),
        M(1, 40),
        M(1, 40),
        M(1, 33),
        M(1, 33),
        M(1, 33),
    ]
}

pub fn reel_strips() -> Vec<Vec<S>> {
    let line1 = || {
        vec![
            S(0),
            S(8),
            S(1),
            S(5),
            S(2),
            S(3),
            S(4),
            S(6),
            S(1),
            S(8),
            S(1),
            S(0),
            S(4),
            S(3),
            S(0),
            S(5),
            S(1),
            S(4),
            S(3),
            S(2),
            S(7),
            S(0),
            S(1),
            S(4),
            S(6),
            S(3),
            S(5),
            S(1),
            S(2),
            S(4),
            S(0),
            S(1),
            S(0),
        ]
    };
    let line2 = || {
        vec![
            S(0),
            S(1),
            S(4),
            S(0),
            S(1),
            S(1),
            S(0),
            S(3),
            S(4),
            S(1),
            S(0),
            S(8),
            S(2),
            S(0),
            S(1),
            S(2),
            S(5),
            S(2),
            S(1),
            S(3),
            S(1),
            S(3),
            S(2),
            S(2),
            S(1),
            S(3),
            S(5),
            S(1),
            S(2),
            S(3),
            S(0),
            S(2),
            S(0),
            S(1),
            S(7),
            S(0),
            S(2),
            S(6),
            S(2),
            S(0),
        ]
    };
    let line3 = || {
        vec![
            S(0),
            S(7),
            S(3),
            S(8),
            S(1),
            S(5),
            S(3),
            S(2),
            S(3),
            S(4),
            S(3),
            S(6),
            S(0),
            S(1),
            S(0),
            S(6),
            S(2),
            S(0),
            S(2),
            S(5),
            S(3),
            S(0),
            S(4),
            S(3),
            S(1),
            S(0),
            S(4),
            S(3),
            S(6),
            S(0),
            S(1),
            S(4),
            S(2),
        ]
    };
    vec![
        line1(),
        line1(),
        line1(),
        line2(),
        line2(),
        line2(),
        line3(),
        line3(),
        line3(),
    ]
}

pub fn normal_pay_table() -> PayTable {
    hashmap!(
        S(0) => hashmap!(3 => 10),
        S(1) => hashmap!(3 => 20),
        S(2) => hashmap!(3 => 30),
        S(3) => hashmap!(3 => 40),
        S(4) => hashmap!(3 => 80),
        S(5) => hashmap!(3 => 100),
        S(6) => hashmap!(3 => 100),
        S(7) => hashmap!(3 => 100),
    )
}

pub fn floating_pay_table() -> PayTable {
    hashmap!(S(8) => hashmap!(3 => 200, 2 => 10, 1 => 2))
}

#[derive(Debug)]
pub struct CalcResult {
 ///   pub line: usize,
    pub symbol: S,
    pub count: usize,
    pub mul: u16,
}

impl CalcResult {
    pub fn new(parse_result: &ParseResult, mul: u16) -> CalcResult {
        CalcResult {
            mul,
            symbol: parse_result.symbol,
            count: parse_result.count,
        }
    }
}

fn parse_floating_symbol(line: &Vec<S>) -> ParseResult {
    let symbol = S(8);
    let count = line.iter().filter(|s| **s == symbol).count();
    ParseResult { symbol, count }
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::reels;
    use utils::lines;
    use utils::subst;
    use utils::calc;

    #[test]
    fn test_random_spin() {
        let r = reels::random_spin(&reel_metas(), &reel_strips());
        println!("{:?}", r);

        let r = lines::result_lines(&lines(), &r);
        println!("{:?}", r);

        let mut r1 = Vec::new();

        let pt1 = normal_pay_table();
        let pt2 = floating_pay_table();

        for line in r {
            let pr1 = subst::parse_line_without_wild(&line);
            let mr1 = calc::calc_mul(&pt1, &pr1);
            if let Some(mul) = mr1 {
                r1.push(CalcResult::new(&pr1, mul));
            }

            let pr2 = parse_floating_symbol(&line);
            let mr2 = calc::calc_mul(&pt2, &pr2);
            if let Some(mul) = mr2 {
                r1.push(CalcResult::new(&pr2, mul));
            }
        }
        println!("{:?}", r1);
    }
}
