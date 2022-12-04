mod first_challange;
mod second_challange;

pub use utils;

use crate::{first_challange::part1, second_challange::part2};

fn main() {
    println!("Answer to the first challange: {}", part1());
    println!("Answer to the second challange: {}", part2());
}