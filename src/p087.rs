use std::collections::HashSet;

use crate::primes_and_phi::get_all_primes_under;

pub fn p087() {
    let primes = get_all_primes_under(7072);
    let triples = get_all_triplets_under(50_000_000, &primes);
    println!("{} triplets found", triples.len());
}

fn get_all_triplets_under(n: i32, primes: &Vec<i32>) -> HashSet<i32> {
    let mut triplets: HashSet<i32> = HashSet::new();
    for sqr_prime in primes {
        let sqr = sqr_prime.pow(2);
        for cub_prime in primes {
            let subtotal = cub_prime.pow(3) + sqr;
            if subtotal >= n {
                break;
            }
            for forth_prime in primes {
                let total = forth_prime.pow(4) + subtotal;
                if total >= n {
                    break;
                } else {
                    triplets.insert(total);
                }
            }
        }
    }
    return triplets;
}