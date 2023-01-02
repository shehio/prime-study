mod miller_rabin;
mod common;
mod fermat;
mod carmichael;
mod korselt;

use crate::miller_rabin::is_composite;
use crate::fermat::is_prime;
use crate::carmichael::is_carmichael;
use crate::korselt::is_carmichael as is_korselt_carmichael;
use crate::common::{divides, gcd, get_factors, is_square, mod_pow, relatively_prime};

fn main() {
    println!("{}", is_composite(23, 2));
    println!("{}", is_prime(23, 2));
    println!("{}", is_prime(1, 2));

    // Not only a Fermat's Liar, but also a Carmichael number
    println!("{}", is_prime(561, 561));

    println!("{}", is_carmichael(561));

    println!("{}", is_square(9));
    println!("{}", is_square(23));


    println!("{:?}", get_factors(23));
    println!("{:?}", get_factors(9));
    println!("{:?}", get_factors(120));


    println!("{}", is_korselt_carmichael(120));

    println!("{}", divides(2, 560));
    println!("{}", is_korselt_carmichael(561));
}
