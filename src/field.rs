use crate::constants::*;
// use constants::*;
use colored::*;
use std::io::{self, Write};
//use proconio::*;


pub fn get_index_by_position(y: u8, x: u8) -> usize {
    POSITION_TO_INDEX[y as usize][x as usize]
}
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

    pub fn fall(&mut self) {
        for x in 0..FIELD_WIDTH {
            let mut write_index = POSITION_TO_INDEX[FIELD_HEIGHT - 1][x];
            for y in (0..FIELD_HEIGHT).rev() {
                let index = POSITION_TO_INDEX[y][x];
                if self.cells[index] == 0 { continue }
                if write_index == index { 
                    write_index -= 1;
                    continue;
                 }
                self.cells[write_index] = self.cells[index];
                self.cells[index] = 0;
                write_index -= 1;
            }
        }
    }

    pub fn single_chain(&mut self) -> bool {
        let is_chained: bool = false;


        is_chained
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
        self.cells[DEATH_POSITION] != 0
    }
}


