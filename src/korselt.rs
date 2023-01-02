use crate::common::{CompositeKind, divides, get_factors, is_square, Kind, mod_pow, relatively_prime};
use crate::common::CompositeKind::Composite;

use mod_exp::mod_exp;
use crate::common::Kind::{Prime, ProbablyPrime};
use crate::fermat::is_prime;

pub fn is_carmichael(n: u64) -> CompositeKind {
    let factors = get_factors(n);
    let rounds = 5;

    for i in factors {

        // 1 is a square free number and i - 1 is zero, so it can't divide n - 1.
        if i == 1 {
            continue;
        }

        // Testing if n is square free. Note that factors contain n.
        if i != 1 && is_square(i) {
            return Composite
        }

        // todo: replace the probabilistic test with a brute-force test.
        if [ProbablyPrime, Prime].contains(&is_prime(i, rounds)) && !divides(i - 1, n - 1) {
            return Composite
        }
    }

    return CompositeKind::Carmichael;
}