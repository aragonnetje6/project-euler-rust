use std::time::Instant;

use crate::primes_and_phi::get_all_phi_under;

pub fn p070() -> i32 {
    let before = Instant::now();
    let phis = get_all_phi_under(10_000_000);
    let (permutation_n, permutation_phi) = phis
        .iter()
        .filter(|(k, _v)| **k != 1)
        .filter(|(k, v)| is_permutation(**k, **v))
        .map(|(k, v)| (*k, *v))
        .min_by(|(n, phi), (n2, phi2)| {
            ((*n as f32) / (*phi as f32)).total_cmp(&((*n2 as f32) / (*phi2 as f32)))
        })
        .expect("No value found");
    println!("Done! took {}s", before.elapsed().as_secs_f32());
    println!(
        "Minimum permutation: n={}, Phi(n)={}, ratio={}",
        permutation_n,
        permutation_phi,
        (permutation_n as f32) / (permutation_phi as f32)
    );
    permutation_n
}

fn is_permutation(x: i32, y: i32) -> bool {
    let mut sorted_x: Vec<char> = x.to_string().chars().collect();
    sorted_x.sort_unstable();
    let mut sorted_y: Vec<char> = y.to_string().chars().collect();
    sorted_y.sort_unstable();
    sorted_x == sorted_y
}
