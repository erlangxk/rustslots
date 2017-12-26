use std::collections::HashMap;

pub static LINES: [[(u8, u8); 3]; 8] = [
    [(3,0), (4,0), (5,0)],
    [(0,0), (1,0), (2,0)],
    [(6,0), (7,0), (8,0)],
    [(0,0), (3,0), (6,0)],
    [(1,0), (4,0), (7,0)],
    [(2,0), (5,0), (8,0)],
    [(0,0), (4,0), (8,0)],
    [(2,0), (4,0), (6,0)],
];

pub fn reels()-> [&'static [u8];9]  {
    static LINE1:[u8;33] = [0, 8, 1, 5, 2, 3, 4, 6, 1, 8, 1, 0, 4, 3, 0, 5, 1, 4, 3, 2, 7, 0, 1, 4, 6, 3, 5, 1, 2, 4, 0, 1, 0];
    static LINE2:[u8;40] = [0, 1, 4, 0, 1, 1, 0, 3, 4, 1, 0, 8, 2, 0, 1, 2, 5, 2, 1, 3, 1, 3, 2, 2, 1, 3, 5, 1, 2, 3, 0, 2, 0, 1, 7, 0, 2, 6, 2, 0];
    static LINE3:[u8;33] = [0, 7, 3, 8, 1, 5, 3, 2, 3, 4, 3, 6, 0, 1, 0, 6, 2, 0, 2, 5, 3, 0, 4, 3, 1, 0, 4, 3, 6, 0, 1, 4, 2];
    return [&LINE1, &LINE1, &LINE1, &LINE2, &LINE2, &LINE2, &LINE3, &LINE3, &LINE3];
}

pub  fn normal_pay_table()->HashMap<u8,HashMap<u8, u16>> {
    hashmap!(
        0=> hashmap!(3=>10),
        1=> hashmap!(3=>20),
        2=> hashmap!(3=>30),
        3=> hashmap!(3=>40),
        4=> hashmap!(3=>80),
        5=> hashmap!(3=>100),
        6=> hashmap!(3=>100),
        7=> hashmap!(3=>100),
    )
}

pub fn floating_pay_table()->HashMap<u8,HashMap<u8,u16>> {
    hashmap!(8=>hashmap!(3=>200, 2=>10, 1=>2))
}
