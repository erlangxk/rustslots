use utils::common::{Coord, Symbol as S};
use utils::calc::PayTable;

pub fn lines() -> Vec<Vec<Coord>> {
    vec![
        vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
        vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)],
        vec![(0, 0), (1, 1), (2, 2), (3, 1), (4, 0)],
        vec![(0, 2), (1, 1), (2, 0), (3, 1), (4, 2)],
        vec![(0, 0), (1, 0), (2, 1), (3, 0), (4, 0)],
        vec![(0, 2), (1, 2), (2, 1), (3, 2), (4, 2)],
        vec![(0, 1), (1, 2), (2, 2), (3, 2), (4, 1)],
        vec![(0, 1), (1, 0), (2, 0), (3, 0), (4, 1)],
    ]
}

pub fn reel_strips() -> Vec<Vec<S>> {
    vec![
        vec![
            S(6),
            S(2),
            S(9),
            S(8),
            S(3),
            S(0),
            S(10),
            S(9),
            S(5),
            S(1),
            S(7),
            S(0),
            S(5),
            S(6),
            S(2),
            S(8),
            S(4),
            S(3),
            S(8),
            S(7),
            S(9),
            S(4),
            S(7),
            S(10),
            S(1),
            S(5),
            S(6),
            S(7),
            S(4),
            S(10),
            S(2),
            S(7),
            S(2),
            S(3),
            S(10),
            S(1),
            S(0),
        ],
        vec![
            S(9),
            S(3),
            S(5),
            S(7),
            S(4),
            S(10),
            S(2),
            S(5),
            S(6),
            S(7),
            S(8),
            S(3),
            S(11),
            S(8),
            S(9),
            S(11),
            S(3),
            S(2),
            S(9),
            S(11),
            S(0),
            S(6),
            S(2),
            S(11),
            S(1),
            S(6),
            S(4),
            S(5),
            S(10),
            S(4),
            S(6),
            S(1),
            S(2),
            S(8),
            S(10),
            S(6),
            S(1),
            S(0),
            S(7),
            S(2),
            S(0),
            S(8),
            S(1),
        ],
        vec![
            S(4),
            S(2),
            S(8),
            S(3),
            S(10),
            S(1),
            S(5),
            S(8),
            S(10),
            S(7),
            S(5),
            S(4),
            S(9),
            S(11),
            S(7),
            S(5),
            S(2),
            S(8),
            S(11),
            S(4),
            S(2),
            S(6),
            S(1),
            S(0),
            S(7),
            S(2),
            S(11),
            S(9),
            S(5),
            S(3),
            S(9),
            S(0),
            S(6),
            S(1),
            S(3),
            S(10),
            S(1),
            S(0),
            S(10),
            S(9),
            S(2),
            S(6),
            S(5),
        ],
        vec![
            S(2),
            S(7),
            S(9),
            S(11),
            S(0),
            S(8),
            S(1),
            S(11),
            S(5),
            S(9),
            S(0),
            S(3),
            S(10),
            S(4),
            S(7),
            S(3),
            S(0),
            S(8),
            S(11),
            S(1),
            S(4),
            S(9),
            S(3),
            S(2),
            S(7),
            S(1),
            S(0),
            S(5),
            S(8),
            S(4),
            S(6),
            S(2),
            S(4),
            S(1),
            S(2),
            S(6),
            S(4),
            S(0),
            S(10),
            S(5),
            S(8),
            S(1),
            S(6),
        ],
        vec![
            S(8),
            S(7),
            S(0),
            S(4),
            S(10),
            S(6),
            S(4),
            S(0),
            S(6),
            S(1),
            S(3),
            S(7),
            S(5),
            S(10),
            S(6),
            S(4),
            S(2),
            S(9),
            S(1),
            S(6),
            S(2),
            S(9),
            S(5),
            S(1),
            S(3),
            S(6),
            S(2),
            S(9),
            S(3),
            S(5),
            S(8),
            S(2),
            S(8),
            S(3),
            S(0),
        ],
    ]
}


pub fn normal_pay_table() -> PayTable {
    hashmap!(
        S(0) => hashmap!(5 => 70,  4 => 25, 3 => 10),
        S(1) => hashmap!(5 => 70,  4 => 25, 3 => 10),
        S(2) => hashmap!(5 => 80,  4 => 30, 3 => 10),
        S(3) => hashmap!(5 => 80,  4 => 30, 3 => 10),
        S(4) => hashmap!(5 => 100, 4 => 45, 3 => 15),
        S(5) => hashmap!(5 => 100, 4 => 45, 3 => 15),
        S(6) => hashmap!(5 => 100, 4 => 45, 3 => 15),
        S(7) => hashmap!(5 => 200, 4 => 60, 3 => 20, 2 => 2),
        S(8) => hashmap!(5 => 200, 4 => 60, 3 => 20, 2 => 2),
        S(9) => hashmap!(5 => 300, 4 => 120,3 => 35, 2 => 5),
    )
}

pub fn scatter_pay_table() -> PayTable {
    hashmap!(
        S(10) => hashmap!(5 => 85, 4 => 10, 3 => 3),
    )
}
