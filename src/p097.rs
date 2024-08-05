use mod_exp::mod_exp;

pub fn p097() {
    let x = (mod_exp(2_u128, 7_830_457_u128, 10_000_000_000_u128) * 28433 + 1) % 10_000_000_000;
    println!("answer: {x}");
}
