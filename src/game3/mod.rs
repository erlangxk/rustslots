mod configs;

use utils::common::{MultiLines, ReelMeta, ReelStrips, Spin, Symbol, Wheel};
use utils::subst::parse_line_with_wild;
use utils::calc::{calc_mul, MulResult, PayTable};
use utils::reels::{random_spin, random_spin_replace};
use utils::lines::{reel_metas_with_diff_len, result_lines, LinesResult};
use utils::scatter::count_single_scatter_duplicate;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Game {
    reel_metas_m1: Vec<ReelMeta>,
    reel_strips_m1: ReelStrips,

    reel_metas_m2: Vec<ReelMeta>,
    reel_strips_m2: ReelStrips,

    reel_metas_f1: Vec<ReelMeta>,
    reel_strips_f1: ReelStrips,
    lines: MultiLines,
    normal_pay_table: PayTable,
    mystery_replacement_table: Vec<(f64, Symbol)>,
}

fn scatter_result(wheel: &Wheel) -> u16 {
    let count: u16 = wheel
        .iter()
        .map(|r| {
            count_single_scatter_duplicate(r, &configs::SCATTER_SYMBOL)
        })
        .sum();
    configs::freespin_table(count)
}

fn spin_replace(
    mystery_replacement_table: &Vec<(f64, Symbol)>,
    reel_metas: &Vec<ReelMeta>,
    reel_strips: &ReelStrips,
) -> Wheel {
    let mut rng = thread_rng();
    let replace_map = configs::replace_mystery(rng.next_f64(), mystery_replacement_table);
    random_spin_replace(reel_metas, reel_strips, &replace_map)
}

fn subst(fst: Symbol, snd: Symbol) -> Option<Symbol> {
    if fst == configs::WILD_SYMBOL && snd != configs::SCATTER_SYMBOL {
        Some(snd)
    } else {
        None
    }
}

fn calc_result(result: &LinesResult, pt: &PayTable) -> Vec<MulResult> {
    let mut r1 = Vec::new();
    for symbols in result.0.iter() {
        let (symbol, count) = parse_line_with_wild(&symbols, &subst);
        if let Some(cr) = calc_mul(&pt, symbol, count) {
            r1.push(cr);
        }
    }
    r1
}

fn total_line_win(muls: &Vec<MulResult>, line_bet: f64) -> f64 {
    let tm: u16 = muls.iter().map(|cr| cr.mul).sum();
    (tm as f64) * line_bet
}

impl Game {
    pub fn new() -> Game {
        let reel_lens: &[u8] = &configs::REEL_LENS;

        let reel_strips_m1 = configs::reel_strips_m1();
        let reel_metas_m1 = reel_metas_with_diff_len(&reel_lens, &reel_strips_m1);

        let reel_strips_m2 = configs::reel_strips_m2();
        let reel_metas_m2 = reel_metas_with_diff_len(&reel_lens, &reel_strips_m2);

        let reel_strips_f1 = configs::reel_strips_f1();
        let reel_metas_f1 = reel_metas_with_diff_len(&reel_lens, &reel_strips_f1);
        let mystery_replacement_table = configs::mystery_replacement_table();
        Game {
            reel_strips_m1,
            reel_metas_m1,
            reel_strips_m2,
            reel_metas_m2,
            reel_strips_f1,
            reel_metas_f1,
            lines: configs::lines(),
            normal_pay_table: configs::normal_pay_table(),
            mystery_replacement_table,
        }
    }


    pub fn spin_main(&self) -> (Wheel, u16) {
        let mut rng = thread_rng();
        if rng.next_f64() <= 0.9618 {
            let wheel = spin_replace(
                &self.mystery_replacement_table,
                &self.reel_metas_m1,
                &self.reel_strips_m1,
            );
            (wheel, 0)
        } else {
            let wheel = random_spin(&self.reel_metas_m2, &self.reel_strips_m2);
            let freespins = scatter_result(&wheel);
            (wheel, freespins)
        }
    }

    pub fn spin_feature(&self) -> Wheel {
        spin_replace(
            &self.mystery_replacement_table,
            &self.reel_metas_f1,
            &self.reel_strips_f1,
        )
    }
}

impl Spin for Game {
    fn spin(&self, line_bet: f64) -> (f64, f64) {
        let mut win: f64 = 0_f64;
        let mut feature: u16 = 0;

        let total_bet = line_bet * 25_f64;
        let (r, freespins) = self.spin_main();

        let rls = result_lines(&self.lines, &r);
        let r = calc_result(&rls, &self.normal_pay_table);

        feature += freespins;
        win += total_line_win(&r, line_bet);

        while feature > 0 {
            let r = self.spin_feature();
            let rls = result_lines(&self.lines, &r);
            let r = calc_result(&rls, &self.normal_pay_table);
            win += total_line_win(&r, line_bet);
            feature -= 1;
        }
        (total_bet, win)
    }
}
