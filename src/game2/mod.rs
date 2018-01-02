mod configs;

use utils::common::{MultiLines, ReelMeta, Spin, Symbol};
use utils::subst::parse_line_with_wild;
use utils::calc::{calc_mul, MulResult, PayTable};
use utils::reels::random_spin;
use utils::lines::{reel_metas_with_same_len, result_lines};

static SCATTER_SYMBOL: Symbol = Symbol(10);
static WILD_SYMBOL: Symbol = Symbol(11);

#[inline(always)]
fn is_scatter(s: &Symbol) -> bool {
    *s == SCATTER_SYMBOL
}

fn subst(fst: Symbol, snd: Symbol) -> Option<Symbol> {
    if fst == WILD_SYMBOL && !is_scatter(&snd) {
        Some(snd)
    } else {
        None
    }
}

#[inline(always)]
fn count_scatter(reel: &Vec<Symbol>) -> u16 {
    for s in reel {
        if is_scatter(s) {
            return 1;
        }
    }
    0
}

fn scatter_result(reel_strips: &Vec<Vec<Symbol>>, pt: &PayTable) -> u16 {
    let count: u16 = reel_strips.iter().map(|r| count_scatter(r)).sum();
    if count != 0 {
        calc_mul(pt, SCATTER_SYMBOL, count as usize).map_or(0, |v| v.mul)
    } else {
        0
    }
}

fn calc_result(result: &Vec<Vec<Symbol>>, pt1: &PayTable) -> Vec<MulResult> {
    let mut r1 = Vec::new();
    for symbols in result.iter() {
        let (symbol, count) = parse_line_with_wild(&symbols, &subst);
        if let Some(cr) = calc_mul(&pt1, symbol, count) {
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
    scatter_pay_table: PayTable,
}

impl Spin for Game {
    fn spin(&self, line_bet: f64) -> (f64, f64) {
        let r = random_spin(&self.reel_metas, &self.reel_strips);
        let sm = scatter_result(&r, &self.scatter_pay_table);
        let r = result_lines(&self.lines, &r);
        let r = calc_result(&r, &self.normal_pay_table);
        let tm: u16 = r.iter().map(|cr| cr.mul).sum();
        let total_bet = line_bet * 9_f64;
        (total_bet, line_bet * (tm  as f64) + total_bet * (sm as f64))
    }
}

impl Game {
    pub fn new() -> Game {
        let reel_strips = configs::reel_strips();
        Game {
            reel_metas: reel_metas_with_same_len(3, &reel_strips),
            reel_strips,
            lines: configs::lines(),
            normal_pay_table: configs::normal_pay_table(),
            scatter_pay_table: configs::scatter_pay_table(),
        }
    }
}