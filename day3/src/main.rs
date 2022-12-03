mod first_challange;
mod second_challange;

pub use utils;

use crate::{first_challange::find_common_item, second_challange::find_common_item_two};


fn main() {
    println!("Answer to the first challange: {}", find_common_item());
    println!("Answer to the second challange: {}", find_common_item_two());
}
