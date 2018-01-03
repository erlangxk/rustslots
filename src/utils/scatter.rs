use std::collections::HashMap;
use utils::common::Symbol;

pub type ScatterResult = HashMap<Symbol, u16>;

pub fn count_multi_scatter_duplidate(
    reel: &Vec<Symbol>,
    is_scatter: fn(&Symbol) -> bool,
) -> ScatterResult {
    let mut result = HashMap::new();
    for s in reel {
        if is_scatter(s) {
            let v = result.entry(*s).or_insert(0);
            *v += 1;
        }
    }
    result
}

pub fn count_multi_scatter_unique(
    reel: &Vec<Symbol>,
    is_scatter: fn(&Symbol) -> bool,
) -> ScatterResult {
    let mut result = HashMap::new();
    for s in reel {
        if is_scatter(s) && !result.contains_key(s) {
            result.insert(*s, 1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::common::Symbol;

    #[test]
    fn test_count_scatter() {
        fn is_scatter(s: &Symbol) -> bool {
            *s == Symbol(1) || *s == Symbol(2)
        }
        let reel = vec![Symbol(1), Symbol(1), Symbol(3), Symbol(2)];
        let r = count_multi_scatter_duplidate(&reel, is_scatter);
        assert_eq!(r, hashmap!(Symbol(1) => 2, Symbol(2) => 1));

        let r = count_multi_scatter_unique(&reel, is_scatter);
        assert_eq!(r, hashmap!(Symbol(1) => 1, Symbol(2) => 1));
    }
}
