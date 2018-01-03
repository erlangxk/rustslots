use utils::common::{Coord, Symbol as S};
use utils::calc::PayTable;

pub fn lines() -> Vec<Vec<Coord>> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 0), (1, 0), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 0), (1, 0), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 0), (1, 0), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 0), (1, 0), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 0), (1, 0), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 0), (1, 0), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 0), (1, 0), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 0), (1, 0), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 0), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 0), (1, 0), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 0), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 0), (1, 0), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 0), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 0), (1, 0), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 0), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 0), (1, 0), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 1), (1, 0), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 1), (1, 0), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 0), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 0), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 0), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 0), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 0), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 0), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 0), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 0), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 0), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 1), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 0), (1, 3), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 1), (1, 0), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 0), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 0), (2, 4), (3, 4), (4, 4), (5, 1)],
        vec![(0, 0), (1, 0), (2, 4), (3, 4), (4, 5), (5, 0)],
        vec![(0, 1), (1, 0), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 1), (1, 0), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 1), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 1), (1, 1), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 1), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 1), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 1), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 1), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 1), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 1), (1, 1), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 1), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 1), (1, 1), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 1), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 1), (1, 1), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 1), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 1), (1, 1), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 1), (1, 1), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 2), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 1), (1, 2), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 1), (1, 2), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 2), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 2), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 1), (1, 2), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 2), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 2), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 2), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 2), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 2), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 1), (1, 2), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 2), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 1), (1, 2), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 2), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 1), (1, 2), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 1), (1, 2), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 2), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 2), (1, 0), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 2), (1, 0), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 0), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 0), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 0), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 0), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 0), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 0), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 0), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 0), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 0), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 0), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 0), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 0), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 0), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 0), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 0), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 0), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 2), (1, 0), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 2), (1, 3), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 2), (1, 3), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 2), (1, 3), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 3), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 3), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 3), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 3), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 3), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 3), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 3), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 3), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 3), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 3), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 3), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 3), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 3), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 3), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 0), (1, 1), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 1), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 0), (1, 1), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 0), (1, 1), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 0), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 0), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 0), (5, 0)],
        vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 6), (5, 6)],
        vec![(0, 0), (1, 1), (2, 3), (3, 3), (4, 4), (5, 5)],
        vec![(0, 0), (1, 1), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 0), (2, 4), (3, 4), (4, 5), (5, 6)],
        vec![(0, 0), (1, 0), (2, 3), (3, 4), (4, 5), (5, 6)],
        vec![(0, 0), (1, 1), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 0), (1, 1), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 1), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 0), (1, 1), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 0), (1, 2), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 2), (2, 0), (3, 0), (4, 0), (5, 1)],
        vec![(0, 0), (1, 2), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 0), (1, 2), (2, 0), (3, 0), (4, 1), (5, 2)],
        vec![(0, 0), (1, 2), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 0), (1, 2), (2, 0), (3, 1), (4, 1), (5, 2)],
        vec![(0, 0), (1, 2), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 0), (1, 2), (2, 0), (3, 1), (4, 2), (5, 3)],
        vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 2), (5, 3)],
        vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 3), (5, 4)],
        vec![(0, 0), (1, 2), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 0), (1, 2), (2, 2), (3, 3), (4, 3), (5, 4)],
        vec![(0, 0), (1, 2), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 2), (2, 2), (3, 3), (4, 4), (5, 5)],
        vec![(0, 0), (1, 2), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 0), (1, 2), (2, 4), (3, 4), (4, 4), (5, 5)],
        vec![(0, 0), (1, 2), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 2), (2, 4), (3, 4), (4, 5), (5, 6)],
        vec![(0, 0), (1, 2), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 0), (1, 2), (2, 4), (3, 5), (4, 5), (5, 6)],
        vec![(0, 0), (1, 2), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 3), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 1), (1, 3), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 1), (1, 3), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 1), (1, 3), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 3), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 1), (1, 3), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 1), (1, 3), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 1), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 2), (1, 1), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 2), (1, 1), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 1), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 1), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 1), (2, 1), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 1), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 1), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 1), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 1), (2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 1), (2, 3), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 1), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 1), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 1), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 1), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 1), (2, 4), (3, 5), (4, 5), (5, 5)],
        vec![(0, 2), (1, 1), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 2), (5, 1)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 2), (5, 0)],
        vec![(0, 1), (1, 2), (2, 4), (3, 3), (4, 4), (5, 6)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 3), (5, 2)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 3), (5, 1)],
        vec![(0, 1), (1, 1), (2, 0), (3, 2), (4, 3), (5, 0)],
        vec![(0, 1), (1, 2), (2, 4), (3, 3), (4, 3), (5, 2)],
        vec![(0, 1), (1, 2), (2, 4), (3, 3), (4, 3), (5, 4)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 3), (5, 0)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 3), (5, 4)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 3), (5, 5)],
        vec![(0, 1), (1, 1), (2, 0), (3, 1), (4, 3), (5, 6)],
        vec![(0, 1), (1, 1), (2, 4), (3, 4), (4, 3), (5, 0)],
        vec![(0, 2), (1, 2), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 2), (1, 2), (2, 0), (3, 0), (4, 0), (5, 1)],
        vec![(0, 2), (1, 2), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 2), (1, 2), (2, 0), (3, 0), (4, 1), (5, 2)],
        vec![(0, 2), (1, 2), (2, 0), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2), (1, 2), (2, 0), (3, 1), (4, 1), (5, 2)],
        vec![(0, 2), (1, 2), (2, 0), (3, 1), (4, 2), (5, 2)],
        vec![(0, 2), (1, 2), (2, 0), (3, 1), (4, 2), (5, 3)],
        vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2)],
        vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 3)],
        vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 3), (5, 4)],
        vec![(0, 2), (1, 2), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 2), (1, 2), (2, 2), (3, 3), (4, 3), (5, 4)],
        vec![(0, 2), (1, 2), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 2), (1, 2), (2, 2), (3, 3), (4, 4), (5, 5)],
        vec![(0, 2), (1, 2), (2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(0, 2), (1, 2), (2, 4), (3, 4), (4, 4), (5, 5)],
        vec![(0, 2), (1, 2), (2, 4), (3, 4), (4, 5), (5, 5)],
        vec![(0, 2), (1, 2), (2, 4), (3, 4), (4, 5), (5, 6)],
        vec![(0, 2), (1, 2), (2, 4), (3, 5), (4, 6), (5, 6)],
        vec![(0, 2), (1, 2), (2, 4), (3, 5), (4, 6), (5, 5)],
        vec![(0, 2), (1, 2), (2, 4), (3, 5), (4, 6), (5, 4)],
        vec![(0, 1), (1, 3), (2, 0), (3, 0), (4, 0), (5, 6)],
        vec![(0, 1), (1, 3), (2, 0), (3, 0), (4, 1), (5, 5)],
        vec![(0, 1), (1, 3), (2, 0), (3, 1), (4, 1), (5, 6)],
        vec![(0, 1), (1, 3), (2, 0), (3, 1), (4, 2), (5, 0)],
        vec![(0, 1), (1, 3), (2, 1), (3, 1), (4, 1), (5, 0)],
        vec![(0, 1), (1, 3), (2, 1), (3, 1), (4, 2), (5, 2)],
        vec![(0, 1), (1, 3), (2, 1), (3, 2), (4, 2), (5, 2)],
        vec![(0, 1), (1, 3), (2, 1), (3, 2), (4, 3), (5, 0)],
        vec![(0, 1), (1, 3), (2, 2), (3, 2), (4, 2), (5, 6)],
        vec![(0, 1), (1, 3), (2, 2), (3, 2), (4, 3), (5, 3)],
        vec![(0, 1), (1, 3), (2, 2), (3, 3), (4, 3), (5, 3)],
        vec![(0, 1), (1, 3), (2, 2), (3, 3), (4, 4), (5, 4)],
        vec![(0, 0), (1, 3), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 3), (2, 0), (3, 0), (4, 0), (5, 1)],
        vec![(0, 0), (1, 3), (2, 0), (3, 0), (4, 1), (5, 1)],
        vec![(0, 0), (1, 3), (2, 0), (3, 0), (4, 1), (5, 2)],
        vec![(0, 0), (1, 3), (2, 3), (3, 4), (4, 4), (5, 4)],
        vec![(0, 0), (1, 3), (2, 3), (3, 4), (4, 4), (5, 5)],
        vec![(0, 0), (1, 3), (2, 3), (3, 4), (4, 5), (5, 5)],
        vec![(0, 0), (1, 3), (2, 3), (3, 4), (4, 5), (5, 6)],
        vec![(0, 0), (1, 3), (2, 1), (3, 2), (4, 2), (5, 3)],
        vec![(0, 0), (1, 3), (2, 1), (3, 2), (4, 3), (5, 3)],
    ]
}


pub fn normal_pay_table() -> PayTable {
    hashmap!(
        S(0) => hashmap!(6 => 50,  5 => 15,  4 => 5,  3 => 1),
        S(1) => hashmap!(6 => 50,  5 => 15,  4 => 5,  3 => 1),
        S(2) => hashmap!(6 => 70,  5 => 20,  4 => 8,  3 => 1),
        S(3) => hashmap!(6 => 70,  5 => 20,  4 => 8,  3 => 1),
        S(4) => hashmap!(6 => 80,  5 => 30,  4 => 15, 3 => 2),
        S(5) => hashmap!(6 => 80,  5 => 30,  4 => 15, 3 => 2),
        S(6) => hashmap!(6 => 90,  5 => 50,  4 => 30, 3 => 3),
        S(7) => hashmap!(6 => 90,  5 => 50,  4 => 30, 3 => 3),
        S(8) => hashmap!(6 => 95,  5 => 80,  4 => 70, 3 => 5),
        S(9) => hashmap!(6 => 100, 5 => 90,  4 => 50, 3 => 5),
    )
}

pub fn reel_strips_ma() -> Vec<Vec<S>> {
    vec![
        vec![
            S(3),
            S(1),
            S(6),
            S(6),
            S(2),
            S(5),
            S(0),
            S(8),
            S(4),
            S(5),
            S(7),
            S(1),
            S(8),
            S(4),
            S(12),
            S(12),
            S(2),
            S(0),
            S(9),
            S(9),
            S(1),
            S(6),
            S(2),
            S(2),
            S(7),
            S(0),
            S(0),
            S(9),
            S(3),
            S(6),
            S(2),
            S(4),
            S(6),
            S(1),
            S(1),
            S(4),
            S(4),
        ],
        vec![
            S(1),
            S(6),
            S(2),
            S(0),
            S(7),
            S(4),
            S(2),
            S(5),
            S(4),
            S(0),
            S(11),
            S(4),
            S(0),
            S(0),
            S(12),
            S(12),
            S(12),
            S(1),
            S(8),
            S(8),
            S(3),
            S(8),
            S(7),
            S(7),
            S(1),
            S(1),
            S(9),
            S(6),
            S(2),
            S(7),
            S(0),
            S(3),
            S(7),
            S(3),
            S(6),
            S(2),
            S(0),
            S(4),
            S(6),
            S(8),
            S(8),
            S(3),
            S(4),
            S(7),
            S(5),
            S(0),
            S(5),
            S(1),
            S(9),
            S(9),
            S(4),
            S(4),
        ],
        vec![
            S(2),
            S(4),
            S(5),
            S(5),
            S(6),
            S(2),
            S(3),
            S(3),
            S(6),
            S(4),
            S(11),
            S(0),
            S(1),
            S(4),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(8),
            S(6),
            S(8),
            S(4),
            S(1),
            S(8),
            S(8),
            S(6),
            S(3),
            S(0),
            S(5),
            S(1),
            S(4),
            S(5),
            S(9),
            S(3),
            S(4),
            S(7),
            S(9),
            S(2),
            S(1),
            S(6),
            S(4),
            S(1),
            S(2),
            S(6),
            S(3),
            S(0),
            S(7),
            S(1),
            S(3),
            S(1),
            S(1),
            S(1),
        ],
        vec![
            S(7),
            S(3),
            S(6),
            S(7),
            S(7),
            S(6),
            S(2),
            S(6),
            S(4),
            S(8),
            S(8),
            S(8),
            S(11),
            S(0),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(3),
            S(2),
            S(9),
            S(4),
            S(9),
            S(4),
            S(2),
            S(6),
            S(2),
            S(7),
            S(0),
            S(1),
            S(5),
            S(4),
            S(1),
            S(7),
            S(4),
            S(3),
            S(4),
            S(5),
            S(1),
            S(1),
            S(5),
            S(1),
            S(4),
            S(3),
            S(3),
            S(7),
            S(4),
            S(4),
        ],
        vec![
            S(6),
            S(7),
            S(3),
            S(3),
            S(0),
            S(0),
            S(0),
            S(3),
            S(4),
            S(7),
            S(9),
            S(3),
            S(11),
            S(3),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(5),
            S(6),
            S(1),
            S(0),
            S(8),
            S(8),
            S(2),
            S(2),
            S(8),
            S(3),
            S(7),
            S(6),
            S(3),
            S(4),
            S(9),
            S(9),
            S(7),
            S(6),
            S(0),
            S(4),
            S(5),
            S(6),
            S(0),
            S(5),
            S(2),
            S(3),
            S(3),
            S(9),
            S(4),
            S(4),
            S(1),
            S(1),
            S(1),
            S(0),
            S(0),
            S(0),
            S(0),
        ],
        vec![
            S(6),
            S(7),
            S(3),
            S(3),
            S(0),
            S(0),
            S(0),
            S(3),
            S(4),
            S(7),
            S(9),
            S(3),
            S(11),
            S(3),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(5),
            S(6),
            S(1),
            S(0),
            S(8),
            S(8),
            S(2),
            S(2),
            S(8),
            S(3),
            S(7),
            S(6),
            S(3),
            S(4),
            S(9),
            S(9),
            S(7),
            S(6),
            S(0),
            S(4),
            S(5),
            S(6),
            S(0),
            S(5),
            S(2),
            S(3),
            S(3),
            S(9),
            S(4),
            S(4),
            S(1),
            S(1),
            S(1),
            S(0),
            S(0),
            S(0),
            S(0),
        ],
    ]
}


pub fn reel_strips_mb() -> Vec<Vec<S>> {
    vec![
        vec![
            S(3),
            S(1),
            S(10),
            S(6),
            S(2),
            S(5),
            S(0),
            S(8),
            S(4),
            S(5),
            S(7),
            S(1),
            S(8),
            S(9),
        ],
        vec![
            S(1),
            S(6),
            S(10),
            S(10),
            S(7),
            S(4),
            S(2),
            S(5),
            S(4),
            S(0),
            S(4),
            S(0),
            S(3),
            S(8),
            S(9),
        ],
        vec![
            S(2),
            S(4),
            S(10),
            S(10),
            S(10),
            S(2),
            S(3),
            S(3),
            S(6),
            S(4),
            S(0),
            S(1),
            S(7),
            S(8),
            S(9),
            S(5),
        ],
        vec![
            S(7),
            S(3),
            S(10),
            S(10),
            S(10),
            S(10),
            S(2),
            S(6),
            S(4),
            S(8),
            S(8),
            S(8),
            S(3),
            S(0),
            S(9),
            S(5),
            S(1),
        ],
        vec![
            S(6),
            S(7),
            S(10),
            S(10),
            S(10),
            S(10),
            S(10),
            S(3),
            S(4),
            S(7),
            S(9),
            S(3),
            S(8),
            S(5),
            S(1),
            S(2),
            S(0),
        ],
        vec![
            S(6),
            S(7),
            S(10),
            S(10),
            S(10),
            S(10),
            S(10),
            S(3),
            S(4),
            S(7),
            S(9),
            S(3),
            S(8),
            S(5),
            S(1),
            S(2),
            S(0),
        ],
    ]
}

pub fn reel_strips_f() -> Vec<Vec<S>> {
    vec![
        vec![
            S(3),
            S(1),
            S(6),
            S(6),
            S(2),
            S(5),
            S(0),
            S(8),
            S(4),
            S(5),
            S(7),
            S(1),
            S(8),
            S(4),
            S(12),
            S(12),
            S(2),
            S(0),
            S(9),
            S(9),
            S(1),
            S(6),
            S(2),
            S(2),
            S(7),
            S(0),
            S(0),
            S(9),
            S(3),
            S(6),
            S(2),
            S(4),
            S(6),
            S(1),
            S(1),
            S(4),
            S(4),
        ],
        vec![
            S(1),
            S(6),
            S(2),
            S(0),
            S(7),
            S(4),
            S(2),
            S(5),
            S(4),
            S(0),
            S(0),
            S(4),
            S(0),
            S(0),
            S(12),
            S(12),
            S(12),
            S(1),
            S(8),
            S(8),
            S(3),
            S(8),
            S(7),
            S(7),
            S(1),
            S(1),
            S(9),
            S(11),
            S(11),
            S(11),
            S(6),
            S(2),
            S(7),
            S(0),
            S(3),
            S(7),
            S(3),
            S(6),
            S(2),
            S(0),
            S(4),
            S(6),
            S(8),
            S(8),
            S(3),
            S(4),
            S(7),
            S(5),
            S(0),
            S(5),
            S(1),
            S(9),
            S(9),
            S(4),
            S(4),
        ],
        vec![
            S(2),
            S(4),
            S(5),
            S(5),
            S(6),
            S(2),
            S(3),
            S(3),
            S(6),
            S(4),
            S(4),
            S(0),
            S(1),
            S(4),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(8),
            S(6),
            S(8),
            S(4),
            S(1),
            S(8),
            S(8),
            S(6),
            S(11),
            S(11),
            S(11),
            S(11),
            S(3),
            S(0),
            S(5),
            S(1),
            S(4),
            S(5),
            S(9),
            S(3),
            S(4),
            S(7),
            S(9),
            S(2),
            S(1),
            S(6),
            S(4),
            S(1),
            S(2),
            S(6),
            S(3),
            S(0),
            S(7),
            S(1),
            S(3),
        ],
        vec![
            S(7),
            S(3),
            S(6),
            S(7),
            S(8),
            S(8),
            S(2),
            S(6),
            S(4),
            S(6),
            S(7),
            S(8),
            S(8),
            S(0),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(3),
            S(2),
            S(9),
            S(4),
            S(9),
            S(4),
            S(2),
            S(6),
            S(11),
            S(11),
            S(11),
            S(11),
            S(11),
            S(2),
            S(7),
            S(0),
            S(1),
            S(5),
            S(4),
            S(1),
            S(7),
            S(4),
            S(3),
            S(4),
            S(5),
            S(1),
            S(1),
            S(5),
            S(1),
            S(4),
            S(3),
            S(3),
            S(7),
            S(4),
            S(4),
        ],
        vec![
            S(6),
            S(7),
            S(3),
            S(3),
            S(0),
            S(9),
            S(0),
            S(3),
            S(4),
            S(7),
            S(7),
            S(3),
            S(0),
            S(3),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(5),
            S(6),
            S(1),
            S(0),
            S(8),
            S(8),
            S(11),
            S(11),
            S(11),
            S(11),
            S(11),
            S(11),
            S(2),
            S(2),
            S(8),
            S(3),
            S(7),
            S(6),
            S(3),
            S(4),
            S(9),
            S(9),
            S(7),
            S(6),
            S(0),
            S(4),
            S(5),
            S(6),
            S(0),
            S(5),
            S(2),
            S(3),
            S(3),
            S(9),
            S(4),
            S(4),
        ],
        vec![
            S(6),
            S(7),
            S(3),
            S(3),
            S(0),
            S(9),
            S(0),
            S(3),
            S(4),
            S(7),
            S(7),
            S(3),
            S(0),
            S(3),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(12),
            S(0),
            S(5),
            S(6),
            S(1),
            S(0),
            S(8),
            S(8),
            S(11),
            S(11),
            S(11),
            S(11),
            S(11),
            S(11),
            S(2),
            S(2),
            S(8),
            S(3),
            S(7),
            S(6),
            S(3),
            S(4),
            S(9),
            S(9),
            S(7),
            S(6),
            S(0),
            S(4),
            S(5),
            S(6),
            S(0),
            S(5),
            S(2),
            S(3),
            S(3),
            S(9),
            S(4),
            S(4),
        ],
    ]
}

pub fn mystery_replacement_table() -> Vec<(f64, S)> {
    let ps = [0.23, 0.19, 0.19, 0.18, 0.03, 0.07, 0.025, 0.02, 0.025, 0.04];
    let ss = vec![S(0), S(1), S(2), S(3), S(4), S(5), S(6), S(7), S(8), S(9)];
    let result: Vec<(f64, S)> = ps.iter()
        .scan(0_f64, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .zip(ss)
        .collect();
    result
}

pub fn replace_mystery(v: f64, config: &Vec<(f64, S)>) -> S {
    for i in config {
        if v <= i.0 {
            return i.1;
        }
    }
    S(9)
}

pub fn freespin_table(scatters: u16) -> u16 {
    match scatters {
        10 => 10,
        11 => 15,
        12 => 20,
        13 => 25,
        14 => 30,
        15 => 35,
        16 => 40,
        17 => 45,
        18 => 50,
        19 => 100,
        20 => 250,
        _ => 0,
    }
}
