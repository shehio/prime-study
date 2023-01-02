use std::{cmp, fmt};

#[derive(PartialEq)]
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

pub enum CompositeKind {
    Composite,
    FermatLiar,
    Carmichael,
}

impl fmt::Display for CompositeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompositeKind::Composite => write!(f, "Composite"),
            CompositeKind::FermatLiar => write!(f, "FermatLiar"),
            CompositeKind::Carmichael => write!(f, "Carmichael")
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

pub fn relatively_prime(m: u64, n: u64) -> bool {
    return gcd(m, n) == 1;
}

pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    let mut minimum = cmp::min(m, n);

    while minimum > 0 {
        if m % minimum == 0 && n % minimum == 0 {
            break;
        }

        minimum -= 1;
    }

    return minimum;
}

pub fn is_square(mut m: u64) -> bool {
    let m_64 = m as f64;
    let sqrt = m_64.sqrt() as u64;
    return sqrt * sqrt == m;
}

// inefficient implementation
pub fn get_factors(mut m: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    for i in 1 .. (m + 1) {
        if m % i == 0 {
            vec.push(i)
        }
    }

    return vec;
}

pub fn divides(m: u64, n: u64) -> bool {
    return n % m == 0;
}