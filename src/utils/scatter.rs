use std::collections::HashMap;
use utils::common::Symbol;

pub type ScatterResult = HashMap<Symbol, u16>;

pub fn count_single_scatter_unique(reel: &Vec<Symbol>, symbol: &Symbol) -> u16 {
    for s in reel {
        if *s == *symbol {
            return 1;
        }
    }
    0
}

pub fn count_single_scatter_duplicate(reel: &Vec<Symbol>, symbol: &Symbol) -> u16 {
    let mut result: u16 = 0;
    for s in reel {
        if *s == *symbol {
            result += 1;
        }
    }
    result
}

pub fn count_multi_scatter_duplicate(
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
    fn test_count_multi_scatter() {
        fn is_scatter(s: &Symbol) -> bool {
            *s == Symbol(1) || *s == Symbol(2)
        }
        let reel = vec![Symbol(1), Symbol(1), Symbol(3), Symbol(2)];
        let r = count_multi_scatter_duplicate(&reel, is_scatter);
        assert_eq!(r, hashmap!(Symbol(1) => 2, Symbol(2) => 1));

        let r = count_multi_scatter_unique(&reel, is_scatter);
        assert_eq!(r, hashmap!(Symbol(1) => 1, Symbol(2) => 1));
    }

    #[test]
    fn test_count_single_scatter() {
        let scatter = Symbol(1);
        let reel = vec![Symbol(1), Symbol(1), Symbol(3), Symbol(2)];
        let r = count_single_scatter_unique(&reel, &scatter);
        assert_eq!(r, 1);
        let r = count_single_scatter_duplicate(&reel, &scatter);
        assert_eq!(r, 2);
    }
}
