pub mod first_challange;

pub use utils;

use crate::first_challange::calc_score;

fn main() {
    println!("Answer to first challange: {}", calc_score());
}
