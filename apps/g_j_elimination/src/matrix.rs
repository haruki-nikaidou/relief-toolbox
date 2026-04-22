use num::{Rational64, Zero};
use smallvec::SmallVec;
use std::rc::Rc;

#[derive(Clone)]
pub struct Row(pub Rc<[Rational64]>);

impl Row {
    pub fn leading_zeros(&self) -> usize {
        self.0.iter().take_while(|&x| x.is_zero()).count()
    }
    pub fn zeros(&self) -> usize {
        self.0.iter().filter(|x| x.is_zero()).count()
    }
    pub fn add_up(&self, other: &Row, factor: Rational64) -> Row {
        let new_row: Rc<[_]> = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b * factor)
            .collect();
        Row(new_row)
    }
}

#[derive(Clone)]
pub struct Matrix(pub SmallVec<[Row; 10]>);

impl Matrix {
    pub fn row_add(&self, from: usize, to: usize, factor: Rational64) -> Matrix {
        let from_row = self.0[from].clone();
        let from_row = from_row.0.iter().map(|item| *item * factor);
        let to_row = self.0[to].clone();
        let new_row: Rc<[_]> = to_row.0.iter().zip(from_row).map(|(a, b)| *a + b).collect();
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

fn human_computing_complexity(a: Rational64) -> i32 {
    if a.is_zero() {
        0
    } else {
        let (p, q) = a.into_raw();
        let p_digits = (p.abs() as f64).log10().ceil() as i32;
        let q_digits = (q.abs() as f64).log10().ceil() as i32;
        p_digits + q_digits
    }
}

/// Evaluate a row add operation is like human or not
///
/// - `zero`: after the operation, how many non-zero elements becomes zero. Negative if some zero elements becomes non-zero.
/// - `head_shift`: after the operation, how many leading zeros are added to the rows
/// - `complexity`: the sum of `ceil(log_10 p) + ceil(log_10 q)` where `p/q` is each element.
pub fn eval_humanity(zero: i32, head_shift: i32, complexity: i32) -> i32 {
    100 * zero - 2 * head_shift - 10 * complexity
}

pub fn eval_row_best_add(from: &Row, to: &Row) -> (Rational64, i32) {
    let to_leading_zero = to.leading_zeros();
    let to_zeros = to.zeros();
    let to_complexity =
        to.0.iter()
            .map(|x| human_computing_complexity(*x))
            .sum::<i32>();
    from.0
        .iter()
        .zip(to.0.iter())
        .map(|(from_tiem, to_item)| {
            if from_tiem.is_zero() || to_item.is_zero() {
                Rational64::zero()
            } else {
                to_item / from_tiem
            }
        })
        .map(|factor| {
            if factor.is_zero() {
                (factor, 0)
            } else {
                let new_to = to.add_up(from, factor);
                let new_leading_zero = new_to.leading_zeros();
                let new_zeros = new_to.zeros();
                let new_complexity = new_to
                    .0
                    .iter()
                    .map(|x| human_computing_complexity(*x))
                    .sum::<i32>();
                let eval = eval_humanity(
                    (new_zeros as i32) - (to_zeros as i32),
                    (new_leading_zero as i32) - (to_leading_zero as i32),
                    new_complexity - to_complexity,
                );
                (factor, eval)
            }
        })
        .max_by_key(|(_, eval)| *eval)
        .unwrap_or((Rational64::zero(), 0))
}

/// Detect a row add operation that can be performed on the given matrix and has a positive humanity score.
/// If all row add operations have a non-positive humanity score, return None.
fn detect_row_add(matrix: &Matrix) -> Option<MatrixOperation> {
    matrix
        .0
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            matrix
                .0
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != row_idx)
                .map(|(add_to_idx, add_to_row)| {
                    let (factor, eval) = eval_row_best_add(row, add_to_row);
                    (add_to_idx, factor, eval)
                })
                .max_by_key(|(_, _, eval)| *eval)
                .map(|(add_to_idx, factor, eval)| (row_idx, add_to_idx, factor, eval))
        })
        .max_by_key(|opt| opt.map(|(_, _, _, eval)| eval).unwrap_or(0))
        .flatten()
        .and_then(|(from_row, to_row, factor, eval)| {
            if eval < 0 {
                None
            } else {
                Some(MatrixOperation::RowAdd {
                    from_row,
                    to_row,
                    factor,
                    after: Box::new(matrix.row_add(from_row, to_row, factor)),
                })
            }
        })
}

pub fn gauss_jordan_elimination(matrix: Matrix) -> Vec<MatrixOperation> {
    todo!()
}
