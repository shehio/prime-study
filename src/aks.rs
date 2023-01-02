use crate::common::Kind;

pub fn is_prime(n: u128) -> Kind {
    let coefficients = get_binomial(n);
    for i in 1 .. coefficients.len() - 1 {
        if coefficients[i] % n != 0 {
            return Kind::Composite;
        }
    }

    return Kind::Prime;
}

fn get_binomial(n: u128) -> Vec<u128> {
    let mut vec = Vec::new();
    for i in 0 .. (n + 1) {
        vec.push(get_binomial_coefficient(n, i))
    }

    return vec;
}

fn get_binomial_coefficient(mut n: u128, mut k: u128) -> u128 {
    if k > n - k {
        k = n - k
    }

    let mut coefficient = 1;
    for i in 0 .. k {
        coefficient *= (n - i);
        coefficient /= (i + 1);
    }

    return coefficient
}