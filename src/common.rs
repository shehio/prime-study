use std::fmt;

pub enum Kind {
    Composite,
    Prime,
    ProbablyPrime,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Composite => write!(f, "Composite"),
            Kind::Prime => write!(f, "Prime"),
            Kind::ProbablyPrime => write!(f, "ProbablyPrime")
        }
    }
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}