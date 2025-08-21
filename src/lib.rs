#![allow(unused)]

struct Matrix<T> {
    matrix_vec: Vec<T>,
    cols: usize,
    rows: usize
}

impl Matrix<u32> {
    fn build_zeros(cols: usize, rows: usize) -> Self {
        let matrix_vec = vec![0; cols * rows];
        Self { matrix_vec, cols, rows }
    }
}

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

