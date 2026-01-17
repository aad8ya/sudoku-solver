pub const BOARD_SIZE: usize = 81;
pub const SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct SudokuBoard {
    cells: [u8; BOARD_SIZE],
}

impl SudokuBoard {
    pub fn new(input: &str) -> Option<Self> {
        if input.len() != BOARD_SIZE {
            return None;
        }

        let mut cells = [0u8; BOARD_SIZE];

        for (i, c) in input.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                cells[i] = digit as u8;
            } else {
                cells[i] = 0;
            }
        }

        Some(SudokuBoard { cells})
    }

    pub fun get_idx(row: usize, col: usize) -> usize {
        row * SIZE + col
    }

    pub fn get_coords(idx: usize) -> (usize, usize) {
        (idx / SIZE, idx % SIZE)
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.cells[Self::get_idx(row, col)]
    }

    pub fn set(&self, row: usize, col: usize, val:u8) {
        let idx = Self::get_idx(row, col);

        self.cells[idx] = val;
    }

    pub fn get_box_idx(row: usize, col: usize) -> usize {
        (row / BOX_SIZE) * BOX_SIZE + (col / BOX_SIZE)
    }
}