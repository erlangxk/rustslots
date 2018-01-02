mod configs;

use utils::common::{MultiLines, ReelMeta, Spin, Symbol};
use utils::subst::{parse_line_without_wild, ParseResult};
use utils::calc::{calc_mul, MulResult, PayTable};
use utils::reels::random_spin;
use utils::lines::{reel_metas_with_same_len, result_lines};

static FLOATING_SYMBOL: Symbol = Symbol(8);

fn parse_floating_symbol(line: &Vec<Symbol>) -> ParseResult {
    let count = line.iter().filter(|s| **s == FLOATING_SYMBOL).count();
    (FLOATING_SYMBOL, count)
}

fn calc_result(result: &Vec<Vec<Symbol>>, pt1: &PayTable, pt2: &PayTable) -> Vec<MulResult> {
    let mut r1 = Vec::new();
    for symbols in result.iter() {
        let (symbol, count) = parse_line_without_wild(&symbols);
        if let Some(cr) = calc_mul(&pt1, symbol, count) {
            r1.push(cr);
        }
        let (symbol, count) = parse_floating_symbol(&symbols);
        if let Some(cr) = calc_mul(&pt2, symbol, count) {
            r1.push(cr);
        }
    }
    r1
}

#[derive(Debug)]
pub struct Game {
    reel_metas: Vec<ReelMeta>,
    reel_strips: Vec<Vec<Symbol>>,
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
        let reel_strips = configs::reel_strips();
        Game {
            reel_metas: reel_metas_with_same_len(1, &reel_strips),
            reel_strips,
            lines: configs::lines(),
            normal_pay_table: configs::normal_pay_table(),
            floating_pay_table: configs::floating_pay_table(),
        }
    }
}
