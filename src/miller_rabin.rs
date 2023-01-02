use rand::Rng;
use crate::common::{Kind, mod_pow};

pub fn is_composite(n: u32, rounds: u32) -> Kind {
    let n_64 = n as u64;

    let (d, s) = find_s_d(n);
    let mut rng = rand::thread_rng();

    for _i in 0..rounds {
        let a: u64 = rng.gen_range(2 .. n_64 - 2);
        let mut x = mod_pow(a, d as u64, n_64);
        let mut y = 0;

        for _j in 0..s {
            y = mod_pow(x, 2, n_64);
            if y == 1 && x != 1u64 && x != (n_64 - 1) {
                return Kind::Composite;
            }
            x = y;
        }

        if y != 1 {
            return Kind::Composite;
        }
    }

    return Kind::ProbablyPrime;
}

fn find_s_d(n: u32) -> (u32, u32) {
    let mut s = n - 1;
    let mut d = 0;
    loop {
        if s % 2 == 0 {
            s /= 2;
            d += 1;
        } else {
            return (s, d);
        }
    }
}
