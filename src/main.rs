mod miller_rabin;
mod common;
mod fermat;

use crate::miller_rabin::is_composite;
use crate::fermat::is_prime;

fn main() {
    println!("{}", is_composite(23, 2));
    println!("{}", is_prime(23, 2));


    // Not only a Fermat's Liar, but also a Carmichael number
    println!("{}", is_prime(561, 561));
}
