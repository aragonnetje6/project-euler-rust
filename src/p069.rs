use std::collections::HashMap;
use std::time::Instant;

use crate::primes_and_phi::get_all_phi_under;

fn get_phi_ratios_under(n: i32) -> HashMap<i32, f32> {
    return HashMap::from_iter(
        get_all_phi_under(n)
            .iter()
            .map(|(k, v)| (k.clone(), *k as f32 / *v as f32))
            .collect::<Vec<_>>(),
    );
}

fn get_max_phi_ratio_under(n: i32) -> (i32, f32) {
    let ratios = get_phi_ratios_under(n);
    let (max_n, max_ratio) = ratios
        .iter()
        .max_by(|(_k1, v1), (_k2, v2)| v1.partial_cmp(v2).expect("NAN or INF found"))
        .expect("maximum fail");
    return (max_n.clone(), max_ratio.clone());
}

pub fn p069() {
    let before = Instant::now();
    let (max_n, max_ratio) = get_max_phi_ratio_under(1000001);
    println!("Done! took {}s", before.elapsed().as_secs_f32());
    println!("N={}, ratio={}", max_n, max_ratio);
}
