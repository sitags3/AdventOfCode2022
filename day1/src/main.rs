use crate::first_challange::find_most_calories;
use crate::second_challange::find_top_3_calories;
pub use utils;

pub mod first_challange;
pub mod second_challange;

fn main() {
    println!("Answer for the first challange: {}", find_most_calories());
    println!("Answer for the second challange: {}", find_top_3_calories())
}
