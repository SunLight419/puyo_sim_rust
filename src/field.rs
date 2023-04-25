use crate::constants::*;
// use constants::*;
use colored::*;
use std::io::{self, Write};

pub struct Field {
    cells: [u8; 78],
}

impl Field {
    pub fn new(cells: [u8; 78]) -> Result<Self, &'static str> {
        if cells.iter().all(|&value| value <= 7) {
            Ok(Field { cells })
        } else {
            Err("Invalid cells: elements must be in the range 0 to 7")
        }
    }

    pub fn show(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for i in 0..FIELD_HEIGHT {
            for j in 0..FIELD_WIDTH {
                if self.cells[i + j * FIELD_HEIGHT as usize] == 0 {
                    print!("{} ", " ");
                    continue;
                }
                let color = match self.cells[i + j * FIELD_HEIGHT] {
                    1 => "red",
                    2 => "green",
                    3 => "cyan",
                    4 => "yellow",
                    5 => "magenta",
                    6 => "white",
                    _ => "black",
                };
                print!("{} ", "●".color(color));
            }
            print!("\n");
        }
        handle.flush().unwrap();
        println!();
    }

    pub fn is_dead(&self) -> bool {
        self.cells[27] != 0
    }
}

pub fn get_index_by_position(x: u8, y: u8) -> usize {
    PRECOMPUTED_INDICES[y as usize][x as usize]
}
