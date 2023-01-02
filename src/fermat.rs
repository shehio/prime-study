use rand::Rng;
use crate::common::{Kind, mod_pow};

pub fn is_prime(n: u64, rounds: u32) -> Kind {
    let mut rng = rand::thread_rng();

    if n == 1 || n == 4 {
        return Kind::Composite;
    }
    if n <= 3 {
        return  Kind::Prime
    }

    for _i in 0..rounds {
        let a = rng.gen_range(2 .. n - 2);
        if mod_pow(a, n - 1, n) == 1 {
            return Kind::ProbablyPrime
        }
    }

    return Kind::Composite;
}