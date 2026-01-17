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

    pub fn get_idx(row: usize, col: usize) -> usize {
        row * SIZE + col
    }

    pub fn get_coords(idx: usize) -> (usize, usize) {
        (idx / SIZE, idx % SIZE)
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.cells[Self::get_idx(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, val:u8) {
        let idx = Self::get_idx(row, col);

        self.cells[idx] = val;
    }

    pub fn get_box_idx(row: usize, col: usize) -> usize {
        (row / BOX_SIZE) * BOX_SIZE + (col / BOX_SIZE)
    }

    pub fn get_candidates(&self, row: usize, col: usize) -> u16 {
        if self.get(row, col) != 0 {
            return 0;
        }

        let mut mask: u16 = 0x3FE;

        let box_row_start = (row / BOX_SIZE) * BOX_SIZE;
        let box_col_start = (col / BOX_SIZE) * BOX_SIZE;

        for i in 0..SIZE {
            let row_val = self.get(row, i);
            if row_val != 0 { mask &= !(1 << row_val); }

            let col_val = self.get(i, col);
            if col_val != 0 { mask &= !(1 << col_val); }

            let r = box_row_start + (i / BOX_SIZE);
            let c = box_col_start + (i % BOX_SIZE);
            let box_val = self.get(r, c);
            if box_val != 0 { mask &= !(1 << box_val); }
        }

        mask
    }
}