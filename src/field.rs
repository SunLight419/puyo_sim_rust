use crate::constants::*;
use std::collections::VecDeque;
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
                if self.cells[index] == 0 {
                    continue;
                }
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

    pub fn is_chain(&self) -> bool {
        let mut visited = [false; 78];

        for x in 0..FIELD_WIDTH {
            for y in 1..FIELD_HEIGHT {
                let current_index = POSITION_TO_INDEX[y][x];
                if visited[current_index]{
                    continue;
                }
                if self.cells[current_index] == 0  || self.cells[current_index] == 6 {
                    continue;
                }
                let mut que: VecDeque<usize> = VecDeque::new();
                let mut connected_puyos = 0;
                que.push_back(current_index);
                visited[current_index] = true;

                while !que.is_empty() {
                    let index = que.pop_front().unwrap();
                    connected_puyos += 1;
                    if index % FIELD_HEIGHT > 1 && !visited[index - 1] && self.cells[index - 1] == self.cells[index] {
                        que.push_back(index - 1);
                        visited[index - 1] = true;
                    }
                    if index % FIELD_HEIGHT < FIELD_HEIGHT - 1 && !visited[index + 1] && self.cells[index + 1] == self.cells[index] {
                        que.push_back(index + 1);
                        visited[index + 1] = true;
                    }
                    if index / FIELD_HEIGHT > 1 && !visited[index - FIELD_HEIGHT] && self.cells[index - FIELD_HEIGHT] == self.cells[index] {
                        que.push_back(index - FIELD_HEIGHT);
                        visited[index - FIELD_HEIGHT] = true;
                    }
                    if index / FIELD_HEIGHT < FIELD_WIDTH - 1 && !visited[index + FIELD_HEIGHT] && self.cells[index + FIELD_HEIGHT] == self.cells[index] {
                        que.push_back(index + FIELD_HEIGHT);
                        visited[index + FIELD_HEIGHT] = true;
                    }
                }
                if connected_puyos >= 4 {
                    return true;
                }
            }
        }

        false
    }

    #[inline]
    pub fn single_chain(&mut self) -> bool {
        let mut is_chained: bool = false;
        let mut visited = [false; 78];
        // let mut vanishing_cells: VecDeque<usize> = VecDeque::new();

        for x in 0..FIELD_WIDTH {
            for y in 1..FIELD_HEIGHT {
                let current_index = POSITION_TO_INDEX[y][x];
                if visited[current_index]{
                    continue;
                }
                if self.cells[current_index] == 0  || self.cells[current_index] == 6 {
                    continue;
                }
                let mut que: VecDeque<usize> = VecDeque::new();
                let mut recorded_index: VecDeque<usize> = VecDeque::new();
                que.push_back(current_index);
                visited[current_index] = true;

                while !que.is_empty() {
                    let index = que.pop_front().unwrap();
                    recorded_index.push_back(index);
                    if index % FIELD_HEIGHT > 1 && !visited[index - 1] && self.cells[index - 1] == self.cells[index] {
                        que.push_back(index - 1);
                        visited[index - 1] = true;
                    }
                    if index % FIELD_HEIGHT < FIELD_HEIGHT - 1 && !visited[index + 1] && self.cells[index + 1] == self.cells[index] {
                        que.push_back(index + 1);
                        visited[index + 1] = true;
                    }
                    if index / FIELD_HEIGHT > 1 && !visited[index - FIELD_HEIGHT] && self.cells[index - FIELD_HEIGHT] == self.cells[index] {
                        que.push_back(index - FIELD_HEIGHT);
                        visited[index - FIELD_HEIGHT] = true;
                    }
                    if index / FIELD_HEIGHT < FIELD_WIDTH - 1 && !visited[index + FIELD_HEIGHT] && self.cells[index + FIELD_HEIGHT] == self.cells[index] {
                        que.push_back(index + FIELD_HEIGHT);
                        visited[index + FIELD_HEIGHT] = true;
                    }
                }
                if recorded_index.len() >= 4 {
                    is_chained = true;

                    while recorded_index.len() > 0 {
                        let index = recorded_index.pop_front().unwrap();
                        self.cells[index] = 0;

                        if index % FIELD_HEIGHT > 1 && self.cells[index - 1] == 6 {
                            self.cells[index - 1] = 0;
                            visited[index - 1] = true;
                        }
                        if index % FIELD_HEIGHT < FIELD_HEIGHT - 1 && self.cells[index + 1] == 6  {
                            self.cells[index + 1] = 0;
                            visited[index + 1] = true;
                        }
                        if index / FIELD_HEIGHT > 1 && self.cells[index - FIELD_HEIGHT] == 6  {
                            self.cells[index - FIELD_HEIGHT] = 0;
                            visited[index - FIELD_HEIGHT] = true;
                        }
                        if index / FIELD_HEIGHT < FIELD_WIDTH - 1 && self.cells[index + FIELD_HEIGHT] == 6 {
                            self.cells[index + FIELD_HEIGHT] = 0;
                            visited[index + FIELD_HEIGHT] = true;
                        }
                    }
                }
            }
        }

        is_chained
    }

    pub fn chain(&mut self) -> u32 {
        let mut chain_count:u32 = 0;

        while self.single_chain() {
            self.fall();
            chain_count += 1;
        }
        chain_count
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
                    _ => "gray",
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
