#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use compact_str::CompactString;
use wasm_bindgen::prelude::*;

use crate::solver::{solve_long, solve_short};

pub mod parser;
pub mod solver;

pub enum ReturnResult {
    Minterm(Vec<CompactString>),
    Boolean(bool),
}

pub fn solve_sat(expr: String) -> Result<ReturnResult, parser::ParseError> {
    let expr = parser::parse(&expr)?;
    let mut vars = smallvec::SmallVec::<[char; 26]>::new();
    expr.collect_variables(&mut vars);
    let mut var_to_id = [0u8; 26];
    for (i, &var) in vars.iter().enumerate() {
        var_to_id[(var as u8 - b'a') as usize] = i as u8;
    }
    let symbols = solver::SymbolTable {
        var_to_id,
        id_to_var: vars.clone(),
        vars,
    };
    let num_vars = symbols.vars.len();
    if num_vars == 0 {
        Err(parser::ParseError::NoVariables)
    } else if num_vars <= 10 {
        let min_terms = solve_short(&expr, &symbols);
        let min_terms = min_terms
            .into_iter()
            .map(|values| symbols.show_values_expr(values))
            .collect();
        Ok(ReturnResult::Minterm(min_terms))
    } else {
        let result = solve_long(&expr, &symbols);
        Ok(ReturnResult::Boolean(result))
    }
}

fn parse_error_message(err: &parser::ParseError) -> String {
    match err {
        parser::ParseError::UnexpectedToken(tok) => {
            format!("Unexpected token: {tok:?}")
        }
        parser::ParseError::UnexpectedChar(c) => {
            format!("Unexpected character: '{c}'")
        }
        parser::ParseError::UnexpectedEndOfInput => {
            "Unexpected end of input".to_string()
        }
        parser::ParseError::NoVariables => {
            "Expression contains no variables".to_string()
        }
    }
}

#[wasm_bindgen(js_name = solveSatJson)]
pub fn solve_sat_json(expr: String) -> String {
    match solve_sat(expr) {
        Ok(ReturnResult::Minterm(ms)) => serde_json::json!({
            "kind": "minterms",
            "values": ms,
        })
        .to_string(),
        Ok(ReturnResult::Boolean(v)) => serde_json::json!({
            "kind": "boolean",
            "value": v,
        })
        .to_string(),
        Err(e) => serde_json::json!({
            "kind": "error",
            "error": parse_error_message(&e),
        })
        .to_string(),
    }
}
