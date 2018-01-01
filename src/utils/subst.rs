use super::common::Symbol;

#[derive(PartialEq, Debug)]
pub struct ParseResult {
    pub symbol: Symbol,
    pub count: usize,
}

fn parse_line<F>(symbols: &[Symbol], subst: &F) -> ParseResult
where
    F: Fn(Symbol, Symbol) -> Option<Symbol>,
{
    let mut symbol = symbols[0];
    let mut count = 1;
    for i in &symbols[1..] {
        if let Some(s) = subst(symbol, *i) {
            symbol = s;
            count += 1;
        } else {
            break;
        }
    }
    ParseResult { symbol, count }
}

fn subst_simple(fst: Symbol, snd: Symbol) -> Option<Symbol> {
    if fst == snd {
        Some(snd)
    } else {
        None
    }
}


fn subst_complex<F>(subst: &F, fst: Symbol, snd: Symbol) -> Option<Symbol>
where
    F: Fn(Symbol, Symbol) -> Option<Symbol>,
{
    subst_simple(fst, snd)
        .or(subst(fst, snd))
        .or(subst(snd, fst))
}


pub fn parse_line_without_wild(symbols: &[Symbol]) -> ParseResult {
    parse_line(symbols, &subst_simple)
}

pub fn parse_line_with_wild<F>(symbols: &[Symbol], subst: &F) -> ParseResult
where
    F: Fn(Symbol, Symbol) -> Option<Symbol>,
{
    parse_line(symbols, &|fst, snd| subst_complex(subst, fst, snd))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn subst(fst: Symbol, snd: Symbol) -> Option<Symbol> {
        if fst == Symbol(8) && snd != Symbol(3) {
            Some(snd)
        } else {
            None
        }
    }

    #[inline(always)]
    fn assert(r: ParseResult, v: u8, count: usize) {
        assert_eq!(
            r,
            ParseResult {
                symbol: Symbol(v),
                count,
            }
        );
    }

    #[inline(always)]
    fn assert_without_wild(line: &[Symbol], v: u8, count: usize) {
        assert(parse_line_without_wild(&line), v, count)
    }

    #[inline(always)]
    fn assert_with_wild(line: &[Symbol], v: u8, count: usize) {
        assert(parse_line_with_wild(&line, &subst), v, count);
    }


    #[test]
    fn test_subst_simple() {
        assert_eq!(None, subst_simple(Symbol(3), Symbol(4)));
        assert_eq!(Some(Symbol(0)), subst_simple(Symbol(0), Symbol(0)));
        assert_eq!(Some(Symbol(3)), subst_simple(Symbol(3), Symbol(3)));
    }


    #[test]
    fn test_subst_complex() {
        assert_eq!(None, subst_complex(&subst, Symbol(8), Symbol(3)));
        assert_eq!(Some(Symbol(0)), subst_complex(&subst, Symbol(8), Symbol(0)));
        assert_eq!(Some(Symbol(0)), subst_complex(&subst, Symbol(0), Symbol(8)));
        assert_eq!(Some(Symbol(4)), subst_complex(&subst, Symbol(8), Symbol(4)));
        assert_eq!(Some(Symbol(4)), subst_complex(&subst, Symbol(4), Symbol(8)));
    }

    #[test]
    fn test_parse_line_without_wild_1() {
        let line = [Symbol(3), Symbol(8), Symbol(3)];
        assert_without_wild(&line, 3, 1);
    }


    #[test]
    fn test_parse_line_without_wild_2() {
        let line = [Symbol(3), Symbol(3), Symbol(8), Symbol(3)];
        assert_without_wild(&line, 3, 2);
    }

    #[test]
    fn test_parse_line_with_wild_1() {
        let line = [Symbol(3), Symbol(8), Symbol(3)];
        assert_with_wild(&line, 3, 1);

        let line = [Symbol(8), Symbol(3), Symbol(3)];
        assert_with_wild(&line, 8, 1);
    }

    #[test]
    fn test_parse_line_with_wild_2() {
        let line = [Symbol(4), Symbol(8), Symbol(3)];
        assert_with_wild(&line, 4, 2);

        let line = [Symbol(8), Symbol(4), Symbol(3)];
        assert_with_wild(&line, 4, 2);

        let line = [Symbol(8), Symbol(4), Symbol(8), Symbol(4), Symbol(5)];
        assert_with_wild(&line, 4, 4);

        let line = [Symbol(8), Symbol(8), Symbol(8), Symbol(4), Symbol(5)];
        assert_with_wild(&line, 4, 4);

        let line = [Symbol(8), Symbol(4), Symbol(4), Symbol(4), Symbol(5)];
        assert_with_wild(&line, 4, 4);

        let line = [Symbol(4), Symbol(4), Symbol(4), Symbol(4), Symbol(5)];
        assert_with_wild(&line, 4, 4);

        let line = [Symbol(4), Symbol(4), Symbol(8), Symbol(8), Symbol(5)];
        assert_with_wild(&line, 4, 4);

        let line = [Symbol(4), Symbol(4), Symbol(4), Symbol(8), Symbol(5)];
        assert_with_wild(&line, 4, 4);
    }
}
