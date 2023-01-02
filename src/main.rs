mod miller_rabin;
mod common;

use crate::miller_rabin::is_composite;

fn main() {
    println!("{}", is_composite(53, 2));
}
