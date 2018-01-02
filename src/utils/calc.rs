use super::common::Symbol;
use std::collections::HashMap;

pub type PayTable = HashMap<Symbol, HashMap<usize, u16>>;

#[derive(Debug)]
pub struct MulResult {
    pub symbol: Symbol,
    pub count: usize,
    pub mul: u16,
}

pub fn calc_mul(table: &PayTable, symbol: Symbol, count: usize) -> Option<MulResult> {
    table.get(&symbol).and_then(|m| m.get(&count)).map(|v| {
        MulResult {
            symbol,
            count,
            mul: *v,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::subst::ParseResult;
    use utils::common::Symbol as S;

    fn pay_table() -> PayTable {
        hashmap!(
            S(0) => hashmap!(3 => 10),
            S(1) => hashmap!(3 => 20),
            S(2) => hashmap!(3 => 30),
            S(3) => hashmap!(3 => 40),
            S(4) => hashmap!(3 => 80),
            S(5) => hashmap!(3 => 100),
            S(6) => hashmap!(3 => 200),
            S(7) => hashmap!(3 => 1000),
            S(8) => hashmap!(3 => 200, 2 => 10, 1 => 2),
        )
    }

    fn calc_mul_test(table: &PayTable, parse_result: &ParseResult) -> Option<u16> {
        calc_mul(table, parse_result).map(|v| v.mul)
    }

    #[test]
    fn test_calc_mul() {
        let pt = pay_table();
        let r = calc_mul_test(&pt, &ParseResult::new(S(8), 2));
        assert_eq!(r, Some(10));

        let r = calc_mul_test(&pt, &ParseResult::new(S(8), 3));
        assert_eq!(r, Some(200));

        let r = calc_mul_test(&pt, &ParseResult::new(S(8), 1));
        assert_eq!(r, Some(2));

        let r = calc_mul_test(&pt, &ParseResult::new(S(3), 3));
        assert_eq!(r, Some(40));

        let r = calc_mul_test(&pt, &ParseResult::new(S(3), 2));
        assert_eq!(r, None);

        let r = calc_mul_test(&pt, &ParseResult::new(S(7), 3));
        assert_eq!(r, Some(1000));

        let r = calc_mul_test(&pt, &ParseResult::new(S(7), 2));
        assert_eq!(r, None);
    }

}
