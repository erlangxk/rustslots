use super::common::{Coord, Line, MultiLines, ReelStrips, Symbol};

pub fn line_def1(raw: &[usize]) -> Line {
    raw.iter().map(|&v| Coord(v, 0)).collect()
}

pub fn line_def2(raw: &[usize]) -> Line {
    raw.iter()
        .enumerate()
        .map(|(index, &v)| Coord(index, v))
        .collect()
}

pub fn result_lines(lines: &MultiLines, reels: &ReelStrips) -> Vec<Vec<Symbol>> {
    let mut result = Vec::new();
    for line in lines {
        let mut lr = Vec::new();
        for c in line {
            lr.push(reels[c.0][c.1]);
        }
        result.push(lr);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_def1() {
        let line = [3, 4, 5];
        assert_eq!(
            vec![Coord(3, 0), Coord(4, 0), Coord(5, 0)],
            line_def1(&line)
        );
    }

    #[test]
    fn test_line_def2() {
        let line = [3, 4, 5];
        assert_eq!(
            vec![Coord(0, 3), Coord(1, 4), Coord(2, 5)],
            line_def2(&line)
        );
    }

    fn lines() -> Vec<Vec<Coord>> {
        use utils::common::Coord as C;
        vec![
            vec![C(3, 0), C(4, 0), C(5, 0)],
            vec![C(2, 0), C(4, 0), C(6, 0)],
        ]
    }

    fn reels() -> Vec<Vec<Symbol>> {
        use utils::common::Symbol as S;
        vec![
            vec![S(0)],
            vec![S(1)],
            vec![S(4)],
            vec![S(7)],
            vec![S(8)],
            vec![S(9)],
            vec![S(0)],
            vec![S(3)],
            vec![S(4)],
        ]
    }

    #[test]
    fn test_result_lines() {
        use utils::common::Symbol as S;
        let r = result_lines(&lines(), &reels());
        assert_eq!(r, vec![vec![S(7), S(8), S(9)], vec![S(4), S(8), S(0)]]);
    }
}
