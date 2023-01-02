use rand::Rng;
use crate::common::{Kind, mod_pow};

pub fn is_prime(n: u32, rounds: u32) -> Kind {
    let n_64 = n as u64;
    let mut rng = rand::thread_rng();

    for _i in 0..rounds {
        let a: u64 = rng.gen_range(2 .. n_64 - 2);
        if mod_pow(a, n_64 - 1, n_64) == 1 {
            return Kind::ProbablyPrime
        }
    }

    return Kind::Composite;
}