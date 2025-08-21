#![allow(unused)]

use rand::prelude::*;

struct Game {
    game_grid: Matrix<u32>
}

impl Game {
    // Initializes the game grid with only zeros and one randomly playced 2 or 4
    fn build(cols: usize, rows: usize) -> Self {
        let mut game = Game { game_grid: Matrix::build_zeros(cols, rows) };
        game.place_value_randomly();
        game
    }

    // Places a 2 or 4 at a randomly chosen zero spot in the game grid
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

    fn compute_line_push<'a>(mut line: Vec<&'a mut u32>) {
        // Iterates over immutable references with indices of all elements except the last one. Filter out zeros
        let line_ref: Vec<&u32> = line.iter().map(|val| &**val).collect();
        let mut pair_find_iter = 
            line_ref
            .iter()
            .take(line_ref.len() - 1)
            .enumerate()
            .filter(|(i, val)| ***val == 0);

        // Collect all the pairs that need to be merged
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        loop {
            if let Some((i, &&val)) = pair_find_iter.next() {
                let &neighbour_val = line_ref[i + 1];
                if val == neighbour_val { pairs.push((i, i + 1)); }
                pair_find_iter.next();
            } else {
                break
            }
        }

        // Merge the pairs and move everything to the start of the line
        for (index_1, index_2) in pairs {
            *line[index_1] *= 2;
            *line[index_2] = 0;
        }

        let line_values: Vec<u32> = line.iter().map(|x| **x).filter(|val| *val != 0).collect();
        for (i, val) in line.iter_mut().enumerate() {
            if line_values.len() > i { **val = line_values[i] }
            else { **val = 0 }
        }
    }

    fn compute_grid_push(&mut self, dir: Direction) {
        // Use the iterators from Matrix and compute_line_push here
    }
}

enum Direction { Left, Right, Up, Down }

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

