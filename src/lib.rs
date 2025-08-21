#![allow(unused)]

use rand::prelude::*;

struct Game {
    game_grid: Matrix<u32>
}

impl Game {
    fn build(cols: usize, rows: usize) -> Self {
        Game { game_grid: Matrix::build_zeros(cols, rows) }
    }

    fn place_value_randomly(&mut self) {
        let mut rng = rand::rng();  

        // Choose a empty (val == 0) spot in the game field at random
        let zero_iter = self.game_grid.matrix_vec.iter_mut().filter(|&&mut val| val == 0);
        let chosen_zero = zero_iter.choose(&mut rng).unwrap();

        // Put either 2 or 4 in the chosen spot
        let mut nums= [2, 4];
        nums.shuffle(&mut rng);
        *chosen_zero = nums[0];
    }
}

// Generic matrix struct that is used as the game grid
struct Matrix<T> {
    matrix_vec: Vec<T>,
    cols: usize,
    rows: usize
}

// Used to initialize a rectangular matrix with only zero entries
impl Matrix<u32> {
    fn build_zeros(cols: usize, rows: usize) -> Self {
        let matrix_vec = vec![0; cols * rows];
        Self { matrix_vec, cols, rows }
    }
}

// Implements useful functions to iterate over single cols or rows and to change single entries
impl<T> Matrix<T> {
    fn overwrite_at(&mut self, col_idx: usize, row_idx: usize, value: T) {
        let cols = self.cols;
        let rows = self.rows;

        if col_idx >= cols || row_idx >= rows { panic!("Col/Row index out of bounds!") }

        self.matrix_vec[col_idx * rows + row_idx] = value;
    }

    fn iter_col_mut(&mut self, col_idx: usize) -> impl Iterator<Item = &mut T> {
        if self.cols <= col_idx {panic!("Column index out of bounds!")}

        let start_idx = col_idx * self.rows;
        let end_idx = start_idx + self.rows;

        self.matrix_vec
            .iter_mut()
            .enumerate()
            .filter(move |(i, val)| *i >= start_idx && *i < end_idx)
            .map(|(i, val)| val)
    }

    fn iter_row_mut(&mut self, row_idx: usize) -> impl Iterator<Item = &mut T> {
        if self.rows <= row_idx {panic!("Row index out of bounds!")}

        let cols = self.cols;

        self.matrix_vec
            .iter_mut()
            .enumerate()
            .filter(move |(i, val)| *i % cols == row_idx)
            .map(|(i, val)| val)
    }
}

