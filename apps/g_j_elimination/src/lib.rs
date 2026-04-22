pub mod matrix;

use matrix::{Matrix, MatrixOperation, Row, gauss_jordan_elimination};
use num::{Rational64, Zero};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct InputMatrix {
    rows: Vec<Vec<[i64; 2]>>,
}

#[derive(Serialize, Clone, Copy)]
struct Rat {
    p: i64,
    q: i64,
}

impl From<Rational64> for Rat {
    fn from(r: Rational64) -> Self {
        Rat {
            p: *r.numer(),
            q: *r.denom(),
        }
    }
}

type MatrixData = Vec<Vec<Rat>>;

fn matrix_to_data(m: &Matrix) -> MatrixData {
    m.0.iter()
        .map(|row| row.0.iter().map(|&r| Rat::from(r)).collect())
        .collect()
}

#[derive(Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum StepData {
    Start {
        matrix: MatrixData,
    },
    RowAdd {
        from_row: usize,
        to_row: usize,
        factor: Rat,
        matrix: MatrixData,
    },
    RowMul {
        target: usize,
        factor: Rat,
        matrix: MatrixData,
    },
    RowReorder {
        a: usize,
        b: usize,
        matrix: MatrixData,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ComputeResult {
    Steps { steps: Vec<StepData> },
    Result { matrix: MatrixData },
    Error { error: String },
}

#[wasm_bindgen(js_name = solveGaussJordanJson)]
pub fn solve_gauss_jordan_json(input: String) -> String {
    let result = compute(&input);
    serde_json::to_string(&result).unwrap_or_else(|e| {
        serde_json::to_string(&ComputeResult::Error {
            error: e.to_string(),
        })
        .unwrap_or_default()
    })
}

fn compute(input: &str) -> ComputeResult {
    let input: InputMatrix = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            return ComputeResult::Error {
                error: e.to_string(),
            }
        }
    };

    if input.rows.is_empty() {
        return ComputeResult::Error {
            error: "Matrix must have at least one row".to_string(),
        };
    }

    let col_count = input.rows[0].len();
    if col_count == 0 {
        return ComputeResult::Error {
            error: "Matrix must have at least one column".to_string(),
        };
    }

    for row in &input.rows {
        if row.len() != col_count {
            return ComputeResult::Error {
                error: "All rows must have the same number of columns".to_string(),
            };
        }
    }

    let matrix_rows: SmallVec<[Row; 10]> = input
        .rows
        .iter()
        .map(|row| {
            let rationals: Rc<[Rational64]> = row
                .iter()
                .map(|&[p, q]| {
                    if q == 0 {
                        Rational64::zero()
                    } else {
                        Rational64::new(p, q)
                    }
                })
                .collect();
            Row(rationals)
        })
        .collect();

    let matrix = Matrix(matrix_rows);
    let ops = gauss_jordan_elimination(matrix);
    let op_count = ops.len();

    if op_count < 100 {
        let steps: Vec<StepData> = ops
            .into_iter()
            .map(|op| match op {
                MatrixOperation::Start(m) => StepData::Start {
                    matrix: matrix_to_data(&m),
                },
                MatrixOperation::RowAdd {
                    from_row,
                    to_row,
                    factor,
                    after,
                } => StepData::RowAdd {
                    from_row,
                    to_row,
                    factor: Rat::from(factor),
                    matrix: matrix_to_data(&after),
                },
                MatrixOperation::RowMul {
                    target,
                    factor,
                    after,
                } => StepData::RowMul {
                    target,
                    factor: Rat::from(factor),
                    matrix: matrix_to_data(&after),
                },
                MatrixOperation::RowReorder { a, b, after } => StepData::RowReorder {
                    a,
                    b,
                    matrix: matrix_to_data(&after),
                },
            })
            .collect();
        ComputeResult::Steps { steps }
    } else {
        let final_matrix = ops
            .back()
            .map(|op| match op {
                MatrixOperation::Start(m) => matrix_to_data(m),
                MatrixOperation::RowAdd { after, .. } => matrix_to_data(after),
                MatrixOperation::RowMul { after, .. } => matrix_to_data(after),
                MatrixOperation::RowReorder { after, .. } => matrix_to_data(after),
            })
            .unwrap_or_default();
        ComputeResult::Result {
            matrix: final_matrix,
        }
    }
}
