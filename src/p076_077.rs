pub fn generate_primes(n: i32) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::with_capacity(n as usize);
    let mut i: i32 = 2;
    while &out.len() < &out.capacity() {
        if is_prime(&out, i) {
            out.push(i);
        }
        i += 1;
    }
    return out;
}

pub fn is_prime(primes: &Vec<i32>, n: i32) -> bool {
    for prime in primes.iter() {
        if n % prime == 0 {
            return false;
        } else if prime.pow(2) > n {
            return true;
        }
    }
    return true;
}

// fn get_summation_options_upto(base: &Vec<i32>, max: i32) -> Vec<i32> {
//     let mut out: Vec<i32> = Vec::with_capacity((max + 1) as usize);
//     out.append(&mut vec![0, 0]);
//     let length = base.len();
//     let mut numbers: Vec<i32> = Vec::with_capacity(length);
//     numbers.copy_from_slice(base);
//     numbers.reverse();
//     for i in 2..(max + 1) {
//
//     }
//     return out;
// }