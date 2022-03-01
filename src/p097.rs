use mod_exp::mod_exp;

pub fn p097() {
    let x = (mod_exp(2 as u128, 7830457 as u128, 10000000000 as u128) * 28433 + 1) % 10000000000;
    println!("answer: {}", x)
}
