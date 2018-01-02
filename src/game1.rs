use utils::common::{Coord as C, MultiLines, ReelMeta as M, Spin, Symbol as S};
use utils::subst::{parse_line_without_wild, ParseResult};
use utils::calc::{calc_mul, CalcResult, PayTable};
use utils::reels::random_spin;
use utils::lines::{reel_metas_with_same_len, result_lines};

fn lines() -> Vec<Vec<C>> {
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

fn reel_strips() -> Vec<Vec<S>> {
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

fn normal_pay_table() -> PayTable {
    hashmap!(
        S(0) => hashmap!(3 => 10),
        S(1) => hashmap!(3 => 20),
        S(2) => hashmap!(3 => 30),
        S(3) => hashmap!(3 => 40),
        S(4) => hashmap!(3 => 80),
        S(5) => hashmap!(3 => 100),
        S(6) => hashmap!(3 => 200),
        S(7) => hashmap!(3 => 1000),
    )
}

fn floating_pay_table() -> PayTable {
    hashmap!(S(8) => hashmap!(3 => 200, 2 => 10, 1 => 2))
}

fn parse_floating_symbol(line: &Vec<S>) -> ParseResult {
    let symbol = S(8);
    let count = line.iter().filter(|s| **s == symbol).count();
    ParseResult { symbol, count }
}

fn calc_result(result: &Vec<Vec<S>>, pt1: &PayTable, pt2: &PayTable) -> Vec<CalcResult> {
    let mut r1 = Vec::new();
    for (line, symbols) in result.iter().enumerate() {
        if let Some(cr) = calc_mul(line, &pt1, &parse_line_without_wild(&symbols)) {
            r1.push(cr);
        }
        if let Some(cr) = calc_mul(line, &pt2, &parse_floating_symbol(&symbols)) {
            r1.push(cr);
        }
    }
    r1
}

#[derive(Debug)]
pub struct Game {
    reel_metas: Vec<M>,
    reel_strips: Vec<Vec<S>>,
    lines: MultiLines,
    normal_pay_table: PayTable,
    floating_pay_table: PayTable,
}

impl Spin for Game {
    fn spin(&self, line_bet: f64) -> (f64, f64) {
        let r = random_spin(&self.reel_metas, &self.reel_strips);
        let r = result_lines(&self.lines, &r);
        let r = calc_result(&r, &self.normal_pay_table, &self.floating_pay_table);
        let tm: u16 = r.iter().map(|cr| cr.mul).sum();
        (line_bet * 8_f64, line_bet * (tm as f64))
    }
}

impl Game {
    pub fn new() -> Game {
        let reel_strips = reel_strips();
        Game {
            reel_metas: reel_metas_with_same_len(1, &reel_strips),
            reel_strips,
            lines: lines(),
            normal_pay_table: normal_pay_table(),
            floating_pay_table: floating_pay_table(),
        }
    }
}
