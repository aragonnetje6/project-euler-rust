use crate::p69::get_all_phi_under;

pub fn p70() {
    let phis = get_all_phi_under(10000000);
    let (permutation_n, permutation_phi) = phis
        .iter()
        .filter(|(k, v)| { is_permutation(**k, **v) })
        .map(|(k, v)| {(*k, *v)})
        .min_by_key(|(n, phi)| {n / phi})
        .expect("No value found");
    println!("Minimum permutation totient: n={}, Phi(n)={}", permutation_n, permutation_phi);
}

fn is_permutation(x: i32, y: i32) -> bool {
    unsafe {
        let mut sorted_x = x.to_string().as_mut_vec().clone();
        sorted_x.sort();
        let mut sorted_y = y.to_string().as_mut_vec().clone();
        sorted_y.sort();
        return sorted_x == sorted_y;
    }
}