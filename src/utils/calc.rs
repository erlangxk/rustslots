use super::common::PayTable;
use super::subst::ParseResult;

pub fn calc_mul(table: &PayTable, parse_result: &ParseResult) -> Option<u16> {
    table
        .get(&parse_result.symbol)
        .and_then(|m| m.get(&parse_result.count))
        .map(|v| *v)
}
