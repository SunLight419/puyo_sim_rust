mod field;
pub use field::*;
mod constants;
// use constants::*;
pub use constants::*;
use std::io::{stdin};

fn main() {
    let values: [u8; 78] = [0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2];

    let field = Field::new(values).expect("Invalid values in the array ");
    field.show();

    println!("Press enter to continue... {} {}",  field.is_dead(), FIELD_HEIGHT);

    stdin().read_line(&mut String::new()).unwrap();
    
}
