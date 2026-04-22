use num::{Rational64, Zero};
use smallvec::SmallVec;
use std::{collections::LinkedList, rc::Rc};

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
    pub fn mul_by(&self, factor: Rational64) -> Row {
        let new_row: Rc<[_]> = self.0.iter().map(|x| *x * factor).collect();
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
    pub fn row_mul(&self, target: usize, factor: Rational64) -> Matrix {
        let target_row = self.0[target].clone();
        let new_row: Rc<[_]> = target_row.0.iter().map(|item| *item * factor).collect();
        let mut new_matrix = self.clone();
        new_matrix.0[target] = Row(new_row);
        new_matrix
    }
    pub fn re_arrange(&self) -> Vec<MatrixOperation> {
        assert!(!self.0.is_empty());
        let mut re_arrange = Vec::with_capacity(self.0.len());
        let cols = self.0[0].0.len();
        // selection sort
        for i in 0..self.0.len() {
            let (max_row, _) = match re_arrange.iter().last() {
                None => self.0.iter().enumerate().skip(i).fold(
                    (i, cols + 1),
                    |(max_row_idx, zeros), (j, row)| {
                        let leading_zeros = row.leading_zeros();
                        if leading_zeros < zeros {
                            (j, leading_zeros)
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
    pub fn is_reduced_row_echelon(&self) -> bool {
        let mut leading_one_cols = Vec::new();
        let mut seen_zero_row = false;

        for row in &self.0 {
            let leading_zeros = row.leading_zeros();

            // Check if this is a zero row
            if leading_zeros == row.0.len() {
                seen_zero_row = true;
                continue;
            }

            // If we've seen a zero row, no non-zero rows can appear after it
            if seen_zero_row {
                return false;
            }

            // Check that the leading entry is 1
            if row.0[leading_zeros] != Rational64::from_integer(1) {
                return false;
            }

            // Check that leading 1 positions are strictly increasing
            if let Some(&last_col) = leading_one_cols.last() {
                if leading_zeros <= last_col {
                    return false;
                }
            }

            leading_one_cols.push(leading_zeros);
        }

        // Check that each column with a leading 1 has zeros everywhere else
        for &col in &leading_one_cols {
            for row in &self.0 {
                let leading_zeros = row.leading_zeros();
                // Skip the row that has the leading 1 in this column
                if leading_zeros == col {
                    continue;
                }
                // Check that all other rows have 0 in this column
                if col < row.0.len() && row.0[col] != Rational64::from_integer(0) {
                    return false;
                }
            }
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
            if eval <= 0 {
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

fn transform_to_reduced_row_echelon_form(matrix: &Matrix) -> Vec<MatrixOperation> {
    let mut operations = Vec::new();
    let mut current_matrix = matrix.clone();

    // Forward elimination: eliminate entries below each pivot
    for i in 0..current_matrix.0.len() {
        // Find the pivot for the current row
        let pivot_col = current_matrix.0[i]
            .0
            .iter()
            .position(|&x| !x.is_zero())
            .unwrap_or(current_matrix.0[i].0.len());

        if pivot_col == current_matrix.0[i].0.len() {
            // This is a zero row, skip it
            continue;
        }

        // Make the leading entry 1 by multiplying the row by the inverse of the leading entry
        let leading_entry = current_matrix.0[i].0[pivot_col];
        if leading_entry != Rational64::from_integer(1) {
            let factor = Rational64::from_integer(1) / leading_entry;
            current_matrix = current_matrix.row_mul(i, factor);
            operations.push(MatrixOperation::RowMul {
                target: i,
                factor,
                after: Box::new(current_matrix.clone()),
            });
        }

        // Eliminate the entries below the leading entry
        for j in (i + 1)..current_matrix.0.len() {
            let factor = -current_matrix.0[j].0[pivot_col];
            if !factor.is_zero() {
                current_matrix = current_matrix.row_add(i, j, factor);
                operations.push(MatrixOperation::RowAdd {
                    from_row: i,
                    to_row: j,
                    factor,
                    after: Box::new(current_matrix.clone()),
                });
            }
        }
    }

    // Backward elimination: eliminate entries above each pivot
    for i in (0..current_matrix.0.len()).rev() {
        // Find the pivot for the current row
        let pivot_col = current_matrix.0[i]
            .0
            .iter()
            .position(|&x| !x.is_zero())
            .unwrap_or(current_matrix.0[i].0.len());

        if pivot_col == current_matrix.0[i].0.len() {
            // This is a zero row, skip it
            continue;
        }

        // Eliminate all entries above the pivot
        for j in 0..i {
            let factor = -current_matrix.0[j].0[pivot_col];
            if !factor.is_zero() {
                current_matrix = current_matrix.row_add(i, j, factor);
                operations.push(MatrixOperation::RowAdd {
                    from_row: i,
                    to_row: j,
                    factor,
                    after: Box::new(current_matrix.clone()),
                });
            }
        }
    }

    // Final reordering to ensure rows are properly sorted by leading zeros
    let re_arrange_ops = current_matrix.re_arrange();
    operations.extend(re_arrange_ops);

    operations
}

pub fn gauss_jordan_elimination(mut matrix: Matrix) -> LinkedList<MatrixOperation> {
    let mut operations = LinkedList::new();
    operations.push_back(MatrixOperation::Start(Box::new(matrix.clone())));

    // Use heuristic algorithm to make the matrix almost a reduced row echelon form
    while let Some(op) = detect_row_add(&matrix) {
        operations.push_back(op.clone());
        match op {
            MatrixOperation::RowAdd { after, .. } | MatrixOperation::RowReorder { after, .. } => {
                matrix = *after;
            }
            _ => unreachable!(),
        }
    }

    // Re-arrange rows to put them in proper order
    let re_arrange_ops = matrix.re_arrange();
    for op in re_arrange_ops {
        match &op {
            MatrixOperation::RowReorder { after, .. } => {
                matrix = *after.clone();
            }
            _ => unreachable!(),
        }
        operations.push_back(op);
    }

    // Complete the transformation to reduced row echelon form
    let rref_ops = transform_to_reduced_row_echelon_form(&matrix);
    operations.extend(rref_ops);

    operations
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_matrix(rows: Vec<Vec<i64>>) -> Matrix {
        let matrix_rows: SmallVec<[Row; 10]> = rows
            .into_iter()
            .map(|row| {
                let rationals: Rc<[Rational64]> = row
                    .into_iter()
                    .map(|val| Rational64::from_integer(val))
                    .collect();
                Row(rationals)
            })
            .collect();
        Matrix(matrix_rows)
    }

    #[test]
    fn test_transform_to_rref_simple() {
        // Matrix:
        // [2, 4, 6]
        // [1, 2, 3]
        let matrix = create_matrix(vec![vec![2, 4, 6], vec![1, 2, 3]]);

        let ops = transform_to_reduced_row_echelon_form(&matrix);

        // Get the final matrix
        let final_matrix = match ops.last() {
            Some(MatrixOperation::RowAdd { after, .. })
            | Some(MatrixOperation::RowMul { after, .. })
            | Some(MatrixOperation::RowReorder { after, .. }) => after.as_ref(),
            _ => &matrix,
        };

        // Check if it's in RREF
        assert!(final_matrix.is_reduced_row_echelon());
    }

    #[test]
    fn test_transform_to_rref_3x3() {
        // Matrix:
        // [1, 2, 3]
        // [2, 4, 8]
        // [1, 3, 5]
        let mut matrix = create_matrix(vec![vec![1, 2, 3], vec![2, 4, 8], vec![1, 3, 5]]);

        // Order the matrix first (transform_to_reduced_row_echelon_form expects ordered rows)
        let re_arrange_ops = matrix.re_arrange();
        for op in re_arrange_ops {
            match op {
                MatrixOperation::RowReorder { after, .. } => {
                    matrix = *after;
                }
                _ => unreachable!(),
            }
        }

        let ops = transform_to_reduced_row_echelon_form(&matrix);

        // Get the final matrix
        let final_matrix = match ops.last() {
            Some(MatrixOperation::RowAdd { after, .. })
            | Some(MatrixOperation::RowMul { after, .. })
            | Some(MatrixOperation::RowReorder { after, .. }) => after.as_ref(),
            _ => &matrix,
        };

        // Check if it's in RREF
        assert!(final_matrix.is_reduced_row_echelon());
    }

    #[test]
    fn test_gauss_jordan_elimination_simple() {
        // Matrix:
        // [2, 4, 6]
        // [1, 2, 3]
        let matrix = create_matrix(vec![vec![2, 4, 6], vec![1, 2, 3]]);

        let operations = gauss_jordan_elimination(matrix);

        // Get the final matrix from the last operation
        let final_matrix = match operations.back() {
            Some(MatrixOperation::RowAdd { after, .. })
            | Some(MatrixOperation::RowMul { after, .. })
            | Some(MatrixOperation::RowReorder { after, .. }) => after.as_ref(),
            Some(MatrixOperation::Start(m)) => m.as_ref(),
            None => panic!("No operations"),
        };

        // Check if the final matrix is in RREF
        assert!(final_matrix.is_reduced_row_echelon());
    }

    #[test]
    fn test_gauss_jordan_elimination_3x4() {
        // Matrix (augmented matrix for a system of equations):
        // [1, 2, 1, 4]
        // [2, 5, 3, 10]
        // [3, 7, 4, 14]
        let matrix = create_matrix(vec![vec![1, 2, 1, 4], vec![2, 5, 3, 10], vec![3, 7, 4, 14]]);

        let operations = gauss_jordan_elimination(matrix);

        // Get the final matrix from the last operation
        let final_matrix = match operations.back() {
            Some(MatrixOperation::RowAdd { after, .. })
            | Some(MatrixOperation::RowMul { after, .. })
            | Some(MatrixOperation::RowReorder { after, .. }) => after.as_ref(),
            Some(MatrixOperation::Start(m)) => m.as_ref(),
            None => panic!("No operations"),
        };

        // Check if the final matrix is in RREF
        assert!(final_matrix.is_reduced_row_echelon());

        // Verify operations list is not empty and starts with Start
        assert!(!operations.is_empty());
        assert!(matches!(
            operations.front(),
            Some(MatrixOperation::Start(_))
        ));
    }

    #[test]
    fn test_is_reduced_row_echelon_true() {
        // Matrix in RREF:
        // [1, 0, 2]
        // [0, 1, 3]
        let rows: SmallVec<[Row; 10]> = vec![
            Row(Rc::from([
                Rational64::from_integer(1),
                Rational64::from_integer(0),
                Rational64::from_integer(2),
            ])),
            Row(Rc::from([
                Rational64::from_integer(0),
                Rational64::from_integer(1),
                Rational64::from_integer(3),
            ])),
        ]
        .into_iter()
        .collect();
        let matrix = Matrix(rows);

        assert!(matrix.is_reduced_row_echelon());
    }

    #[test]
    fn test_is_reduced_row_echelon_false_not_leading_one() {
        // Matrix not in RREF (leading entry is not 1):
        // [2, 0, 4]
        // [0, 1, 3]
        let rows: SmallVec<[Row; 10]> = vec![
            Row(Rc::from([
                Rational64::from_integer(2),
                Rational64::from_integer(0),
                Rational64::from_integer(4),
            ])),
            Row(Rc::from([
                Rational64::from_integer(0),
                Rational64::from_integer(1),
                Rational64::from_integer(3),
            ])),
        ]
        .into_iter()
        .collect();
        let matrix = Matrix(rows);

        assert!(!matrix.is_reduced_row_echelon());
    }

    #[test]
    fn test_is_reduced_row_echelon_false_column_not_zero() {
        // Matrix not in RREF (column with leading 1 has non-zero entry):
        // [1, 2, 3]
        // [0, 1, 4]
        let rows: SmallVec<[Row; 10]> = vec![
            Row(Rc::from([
                Rational64::from_integer(1),
                Rational64::from_integer(2),
                Rational64::from_integer(3),
            ])),
            Row(Rc::from([
                Rational64::from_integer(0),
                Rational64::from_integer(1),
                Rational64::from_integer(4),
            ])),
        ]
        .into_iter()
        .collect();
        let matrix = Matrix(rows);

        assert!(!matrix.is_reduced_row_echelon());
    }
}
