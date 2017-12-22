type Coord = (u8, u8);

pub fn line_def1(raw: &[u8]) -> Vec<Coord> {
    raw.iter().map(|&v| (v, 0)).collect()
}

pub fn line_def2(raw: &[u8]) -> Vec<Coord> {
    raw.iter()
        .enumerate()
        .map(|(index, &v)| (index as u8, v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_def1() {
        let line = [3, 4, 5];
        assert_eq!(vec![(3, 0), (4, 0), (5, 0)], line_def1(&line));
    }

    #[test]
    fn test_line_def2() {
        let line = [3, 4, 5];
        assert_eq!(vec![(0, 3), (1, 4), (2, 5)], line_def2(&line));
    }
}
