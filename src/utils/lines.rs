use super::common::{Coord, Line, MultiLines, Symbol};

pub fn line_def1(raw: &[usize]) -> Line {
    raw.iter().map(|&v| Coord(v, 0)).collect()
}

pub fn line_def2(raw: &[usize]) -> Line {
    raw.iter()
        .enumerate()
        .map(|(index, &v)| Coord(index, v))
        .collect()
}

pub fn result_lines(lines: &MultiLines, reels: &Vec<Vec<Symbol>>) -> Vec<Vec<Symbol>> {
    let mut result = Vec::<Vec<Symbol>>::new();
    for line in lines {
        let mut lr = Vec::<Symbol>::new();
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
}
