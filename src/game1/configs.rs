use utils::common::{Coord, Symbol as S, ReelStrips};
use utils::calc::PayTable;

pub fn reel_strips() -> ReelStrips {
    let line1 = || {
        vec![
            S(0),
            S(8),
            S(1),
            S(5),
            S(2),
            S(3),
            S(4),
            S(6),
            S(1),
            S(8),
            S(1),
            S(0),
            S(4),
            S(3),
            S(0),
            S(5),
            S(1),
            S(4),
            S(3),
            S(2),
            S(7),
            S(0),
            S(1),
            S(4),
            S(6),
            S(3),
            S(5),
            S(1),
            S(2),
            S(4),
            S(0),
            S(1),
            S(0),
        ]
    };
    let line2 = || {
        vec![
            S(0),
            S(1),
            S(4),
            S(0),
            S(1),
            S(1),
            S(0),
            S(3),
            S(4),
            S(1),
            S(0),
            S(8),
            S(2),
            S(0),
            S(1),
            S(2),
            S(5),
            S(2),
            S(1),
            S(3),
            S(1),
            S(3),
            S(2),
            S(2),
            S(1),
            S(3),
            S(5),
            S(1),
            S(2),
            S(3),
            S(0),
            S(2),
            S(0),
            S(1),
            S(7),
            S(0),
            S(2),
            S(6),
            S(2),
            S(0),
        ]
    };
    let line3 = || {
        vec![
            S(0),
            S(7),
            S(3),
            S(8),
            S(1),
            S(5),
            S(3),
            S(2),
            S(3),
            S(4),
            S(3),
            S(6),
            S(0),
            S(1),
            S(0),
            S(6),
            S(2),
            S(0),
            S(2),
            S(5),
            S(3),
            S(0),
            S(4),
            S(3),
            S(1),
            S(0),
            S(4),
            S(3),
            S(6),
            S(0),
            S(1),
            S(4),
            S(2),
        ]
    };
    ReelStrips(vec![
        line1(),
        line1(),
        line1(),
        line2(),
        line2(),
        line2(),
        line3(),
        line3(),
        line3(),
    ])
}


pub fn lines() -> Vec<Vec<Coord>> {
    vec![
        vec![(3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 0), (2, 0)],
        vec![(6, 0), (7, 0), (8, 0)],
        vec![(0, 0), (3, 0), (6, 0)],
        vec![(1, 0), (4, 0), (7, 0)],
        vec![(2, 0), (5, 0), (8, 0)],
        vec![(0, 0), (4, 0), (8, 0)],
        vec![(2, 0), (4, 0), (6, 0)],
    ]
}

pub fn normal_pay_table() -> PayTable {
    hashmap!(
        S(0) => hashmap!(3 => 10),
        S(1) => hashmap!(3 => 20),
        S(2) => hashmap!(3 => 30),
        S(3) => hashmap!(3 => 40),
        S(4) => hashmap!(3 => 80),
        S(5) => hashmap!(3 => 100),
        S(6) => hashmap!(3 => 200),
        S(7) => hashmap!(3 => 1000),
    )
}

pub fn floating_pay_table() -> PayTable {
    hashmap!(S(8) => hashmap!(3 => 200, 2 => 10, 1 => 2))
}
