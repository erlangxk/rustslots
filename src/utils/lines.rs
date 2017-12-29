use super::common::Coord;

pub fn line_def1(raw: &[u16]) -> Vec<Coord> {
    raw.iter().map(|&v| Coord(v, 0)).collect()
}

pub fn line_def2(raw: &[u16]) -> Vec<Coord> {
    raw.iter()
        .enumerate()
        .map(|(index, &v)| Coord(index as u16, v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_def1() {
        let line = [3, 4, 5];
        assert_eq!(vec![Coord(3, 0), Coord(4, 0), Coord(5, 0)], line_def1(&line));
    }

    #[test]
    fn test_line_def2() {
        let line = [3, 4, 5];
        assert_eq!(vec![Coord(0, 3), Coord(1, 4), Coord(2, 5)], line_def2(&line));
    }
}
