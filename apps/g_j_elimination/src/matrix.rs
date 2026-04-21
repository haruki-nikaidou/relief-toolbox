use num::{Rational64, Zero};
use smallvec::SmallVec;
use std::rc::Rc;

#[derive(Clone)]
pub struct Row(pub Rc<[Rational64]>);

impl Row {
    pub fn leading_zeros(&self) -> usize {
        self.0.iter().take_while(|&x| x.is_zero()).count()
    }
}

#[derive(Clone)]
pub struct Matrix(pub SmallVec<[Row; 10]>);

impl Matrix {
    pub fn row_add(&self, from: usize, to: usize, factor: Rational64) -> Matrix {
        let from_row = self.0[from].clone();
        let from_row = from_row.0.iter().map(|item| *item * factor);
        let to_row = self.0[to].clone();
        let new_row: Rc<[_]> = to_row
            .0
            .iter()
            .zip(from_row)
            .map(|(a, b)| *a + b)
            .collect();
        let mut new_matrix = self.clone();
        new_matrix.0[to] = Row(new_row);
        new_matrix
    }
    pub fn row_reorder(&self, a: usize, b: usize) -> Matrix {
        let mut new_matrix = self.clone();
        new_matrix.0.swap(a, b);
        new_matrix
    }
    pub fn re_arrange(&self) -> Vec<MatrixOperation> {
        assert!(!self.0.is_empty());
        let mut re_arrange = Vec::with_capacity(self.0.len());
        let cols = self.0[0].0.len();
        // selection sort
        for i in 0..self.0.len() {
            let (max_row, _) = match re_arrange.iter().last() {
                None => self.0.iter().enumerate().fold(
                    (0, cols + 1),
                    |(max_row_idx, zeros), (i, row)| {
                        let leading_zeros = row.leading_zeros();
                        if leading_zeros < zeros {
                            (i, leading_zeros)
                        } else {
                            (max_row_idx, zeros)
                        }
                    },
                ),
                Some(MatrixOperation::RowReorder { after, .. }) => {
                    after.0.iter().enumerate().skip(i).fold(
                        (i, cols + 1),
                        |(max_row_idx, zeros), (j, row)| {
                            let leading_zeros = row.leading_zeros();
                            if leading_zeros < zeros {
                                (j, leading_zeros)
                            } else {
                                (max_row_idx, zeros)
                            }
                        },
                    )
                }
                _ => unreachable!(),
            };
            if max_row != i {
                re_arrange.push(MatrixOperation::RowReorder {
                    a: i,
                    b: max_row,
                    after: Box::new(self.row_reorder(i, max_row)),
                })
            }
        }
        re_arrange
    }
    pub fn is_row_echelon(&self) -> bool {
        let mut last_leading_zeros = None;
        for row in &self.0 {
            let leading_zeros = row.leading_zeros();
            if let Some(last) = last_leading_zeros {
                if leading_zeros <= last && leading_zeros != row.0.len() {
                    return false;
                }
            }
            last_leading_zeros = Some(leading_zeros);
        }
        true
    }
}

#[derive(Clone)]
pub enum MatrixOperation {
    Start(Box<Matrix>),
    RowAdd {
        from_row: usize,
        to_row: usize,
        factor: Rational64,
        after: Box<Matrix>,
    },
    RowMul {
        target: usize,
        factor: Rational64,
        after: Box<Matrix>,
    },
    RowReorder {
        a: usize,
        b: usize,
        after: Box<Matrix>,
    },
}

/// Evaluate a row add operation is like human or not
///
/// - `zero`: after the operation, how many non-zero elements becomes zero
/// - `head_shift`: after the operation, how many leading zeros are added to the rows
/// - `complexity`: the sum of `ceil(log_10 p) + ceil(log_10 q)` where `p/q` is each element.
pub fn eval_humanity(zero: i32, head_shift: i32, complexity: i32) -> i32 {
    100 * zero - 2 * head_shift - 10 * complexity
}

fn detect_row_add(matrix: &Matrix) -> Option<MatrixOperation> {
    todo!()
}

pub fn gaussian_elimination(matrix: Matrix) -> Vec<MatrixOperation> {
    todo!()
}

pub fn gauss_jordan_elimination(matrix: Matrix) -> Vec<MatrixOperation> {
    todo!()
}
