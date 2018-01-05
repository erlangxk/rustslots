extern crate slots;

use slots::game1::Game;
use slots::utils::common::Spin;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut cost: f64 = 0_f64;
    let mut win: f64 = 0_f64;
    let game = Game::new();

    for _ in 0..1000000 {
        let (c, w) = game.spin(1_f64);
        cost += c;
        win += w;
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Seconds: {}", sec);
    println!("cost is {}, win is {}, ratio is {}", cost, win, win / cost);
}