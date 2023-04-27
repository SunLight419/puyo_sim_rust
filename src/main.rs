mod field;
pub use field::*;
mod constants;
// use constants::*;
pub use constants::*;
pub use std::io::{stdin};
pub use proconio::*;

fn main() {
    let values: [u8; 78] = [
        0, 4, 5, 4, 5, 2, 5, 5, 5, 4, 5, 5, 5,
        0, 3, 3, 3, 2, 3, 2, 2, 4, 2, 4, 4, 4,
        4, 3, 4, 4, 3, 4, 3, 3, 4, 3, 2, 2, 2,
        5, 4, 5, 5, 4, 5, 4, 4, 3, 5, 3, 3, 3,
        2, 5, 2, 2, 5, 2, 5, 5, 4, 3, 5, 5, 5,
        2, 3, 2, 3, 3, 3, 2, 2, 3, 3, 4, 4, 4
        ];

    let mut field = Field::new(values).expect("Invalid values in the array ");
    field.show();
    field.fall();
    field.show();


    println!("Press enter to continue... {} {}",  field.is_dead(), FIELD_HEIGHT);

    let chain_count = field.chain();
    println!("chain count {}", chain_count);
    field.show();

    stdin().read_line(&mut String::new()).unwrap();
    
}
