#![feature(is_sorted)]

use std::time::Instant;
use crate::p69::{get_max_phi_ratio_under};

mod p69;

fn main() {
    let before = Instant::now();
    let (max_n, max_ratio) = get_max_phi_ratio_under(1000001);
    println!("Done! took {}s", before.elapsed().as_secs_f32());
    println!("N={}, ratio={}", max_n, max_ratio);
}
