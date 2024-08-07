#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Fraction {
    n: i32,
    d: i32,
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl Fraction {
    fn reduce(&self) -> Fraction {
        let div = gcd(self.n, self.d);
        Fraction {
            n: self.n / div,
            d: self.d / div,
        }
    }

    fn gcd(&self) -> i32 {
        gcd(self.n, self.d)
    }

    fn is_reduced(&self) -> bool {
        is_reduced(self)
    }

    fn eval(&self) -> f32 {
        self.n as f32 / self.d as f32
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn is_reduced(frac: &Fraction) -> bool {
    gcd(frac.n, frac.d) == 1
}

pub fn p071() -> Fraction {
    const N: i32 = 1_000_000;
    const TARGET: Fraction = Fraction { n: 3, d: 7 };
    let fractions = get_close_fractions_under(TARGET, N);
    fractions
        .into_iter()
        .max_by(|frac1, frac2| frac1.eval().total_cmp(&frac2.eval()))
        .unwrap()
}

fn get_fractions_nominator_under(n: i32) -> Vec<Fraction> {
    let mut out: Vec<Fraction> = Vec::new();
    for i in 2..n {
        for j in 1..i {
            let frac = Fraction { n: j, d: i };
            if is_reduced(&frac) {
                out.push(frac);
            }
        }
    }
    out
}

fn get_close_fractions_under(frac: Fraction, d_max: i32) -> Vec<Fraction> {
    let mut out: Vec<Fraction> = Vec::new();
    let mut other_frac = frac;
    while other_frac.d < d_max {
        other_frac.d += 1;
        other_frac.n += 1;
        if other_frac.eval() > frac.eval() {
            other_frac.n -= 1;
        }
        if other_frac.is_reduced() {
            out.push(other_frac);
        }
    }
    out
}
