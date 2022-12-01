use crate::first_challange::find_most_calories;
pub use utils;

pub mod first_challange;

fn main() {
    println!("Answer: {}", find_most_calories());
}
