mod field;
pub use field::*;
mod constants;
// use constants::*;
pub use constants::*;
pub use std::io::{stdin};
pub use proconio::*;

fn main() {
    let values: [u8; 78] = [
        0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 2, 2,
        6, 7, 7, 1, 2, 3, 4, 5, 6, 7, 3, 2, 6,
        6, 5, 6, 7, 7, 1, 2, 3, 4, 5, 3, 2, 6,
        6, 3, 4, 5, 6, 7, 7, 1, 2, 3, 6, 6, 6,
        6, 1, 2, 3, 4, 5, 6, 7, 6, 1, 2, 6, 6,
        5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 6, 1, 2
        ];

    let mut field = Field::new(values).expect("Invalid values in the array ");
    field.show();
    field.fall();
    field.show();


    println!("Press enter to continue... {} {}",  field.is_dead(), FIELD_HEIGHT);

    if field.is_chain() && (field.is_chain() == field.single_chain()) {
        println!("ok!");
    }
    field.show();

    stdin().read_line(&mut String::new()).unwrap();
    
}
