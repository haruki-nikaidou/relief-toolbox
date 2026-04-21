use num::Rational64;
use smallvec::SmallVec;
use std::rc::Rc;

#[derive(Clone)]
pub struct Row(pub Rc<[Rational64]>);

#[derive(Clone)]
pub struct Matrix(pub SmallVec<[Row; 10]>);

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

pub fn gaussian_elimination(matrix: Matrix) -> Vec<MatrixOperation> {
    todo!()
}

pub fn gauss_jordan_elimination(matrix: Matrix) -> Vec<MatrixOperation> {
    todo!()
}