use crate::common::{CompositeKind, mod_pow, relatively_prime};
use crate::common::CompositeKind::Composite;

use mod_exp::mod_exp;

pub fn is_carmichael(n: u32) -> CompositeKind {
    let n_64 = n as u64;
    let mut liar = false;
    let mut charmichael = true;

    for i in 2 .. n_64 - 1 {
        // This could also be expressed as mod_exp(i, n_64 - 1, n_64) == 1, only for i that is
        // relatively prime to n_64.
        if mod_exp(i, n_64, n_64) == i {
            liar = true;
        } else {
            charmichael = false;
        }

        if liar && !charmichael {
            return CompositeKind::FermatLiar;
        }
    }

    if charmichael {
        return CompositeKind::Carmichael;
    }

    return Composite;
}