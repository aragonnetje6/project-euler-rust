use std::collections::HashMap;

use crate::primes_and_phi::get_all_phi_under;

fn get_phi_ratios_under(n: i32) -> HashMap<i32, f32> {
    return HashMap::from_iter(
        get_all_phi_under(n)
            .iter()
            .map(|(k, v)| (*k, *k as f32 / *v as f32))
            .collect::<Vec<_>>(),
    );
}

fn get_max_phi_ratio_under(n: i32) -> (i32, f32) {
    let ratios = get_phi_ratios_under(n);
    let (max_n, max_ratio) = ratios
        .iter()
        .max_by(|(_k1, v1), (_k2, v2)| v1.partial_cmp(v2).expect("NAN or INF found"))
        .expect("maximum fail");
    (*max_n, *max_ratio)
}

pub fn p069() -> i32 {
    let (max_n, _) = get_max_phi_ratio_under(1_000_001);
    max_n
}
