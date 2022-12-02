pub mod first_challange;
pub mod second_challange;

pub use utils;

use crate::{first_challange::calc_score, second_challange::calc_score_two};

fn main() {
    println!("Answer to the first challange: {}", calc_score());
    println!("Answer to the second challange: {}", calc_score_two());
}
