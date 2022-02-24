use crate::p69::get_all_phi_under;

pub fn p70() {
    let phis = get_all_phi_under(10000000);
    let permutation_phis: Vec<(i32, i32)> = phis.iter().filter(|(k, v)| { is_permutation(**k, **v) }).map(|(k, v)| {(*k, *v)}).collect();
    for (k, v) in permutation_phis {
        println!("{}, {}", k, v);
    }
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