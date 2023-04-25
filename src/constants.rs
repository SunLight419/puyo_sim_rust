pub const FIELD_WIDTH: usize = 6;
pub const FIELD_HEIGHT: usize = 13;
pub const DEATH_POSITION: usize = 27;


const fn create_precomputed_indices() -> [[usize; FIELD_WIDTH]; FIELD_HEIGHT] {
    let mut precomputed_indices = [[0usize; FIELD_WIDTH]; FIELD_HEIGHT];

    let mut y = 0;
    while y < FIELD_HEIGHT {
        let mut x = 0;
        while x < FIELD_WIDTH {
            precomputed_indices[y][x] = (x * FIELD_HEIGHT + y) as usize;
            x += 1;
        }
        y += 1;
    }

    precomputed_indices
}

pub const POSITION_TO_INDEX: [[usize; FIELD_WIDTH]; FIELD_HEIGHT] = create_precomputed_indices();
