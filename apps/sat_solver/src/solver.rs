use compact_str::{CompactString, CompactStringExt, ToCompactString, format_compact};
use smallvec::SmallVec;

use crate::parser::Expression;

pub struct SymbolTable {
    pub var_to_id: [u8; 26],
    pub vars: SmallVec<[char; 26]>,
    pub id_to_var: SmallVec<[char; 26]>,
}

impl SymbolTable {
    pub fn show_values_expr(&self, values: u32) -> CompactString {
        self.vars
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if (values & (1 << i)) != 0 {
                    c.to_compact_string()
                } else {
                    format_compact!("!{c}")
                }
            })
            .collect::<Vec<_>>()
            .join_compact("&")
    }
}

impl Expression {
    pub fn eval(&self, symbols: &SymbolTable, values: u32) -> bool {
        match self {
            Expression::Variable(c) => {
                let id = symbols.var_to_id[(*c as u8 - b'a') as usize];
                (values & (1 << id)) != 0
            }
            Expression::Not(expr) => !expr.eval(symbols, values),
            Expression::And(left, right) => {
                left.eval(symbols, values) && right.eval(symbols, values)
            }
            Expression::Or(left, right) => {
                left.eval(symbols, values) || right.eval(symbols, values)
            }
            Expression::Xor(left, right) => {
                left.eval(symbols, values) ^ right.eval(symbols, values)
            }
        }
    }
    pub fn collect_variables(&self, vars: &mut SmallVec<[char; 26]>) {
        match self {
            Expression::Variable(c) => {
                if !vars.contains(c) {
                    vars.push(*c);
                }
            }
            Expression::Not(expr) => expr.collect_variables(vars),
            Expression::And(left, right)
            | Expression::Or(left, right)
            | Expression::Xor(left, right) => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            }
        }
    }
}

/// Solve the expression and return all satisfying assignments as a vector of bitmasks.
///
/// Call when the number of variables is small (<= 10).
pub fn solve_short(expr: &Expression, symbol_table: &SymbolTable) -> Vec<u32> {
    let mut results = Vec::with_capacity(1 << symbol_table.vars.len());
    let num_vars = symbol_table.vars.len();
    for assignment in 0..(1 << num_vars) {
        if expr.eval(symbol_table, assignment) {
            results.push(assignment);
        }
    }
    results
}

/// Solve the expression and return true if there exists a satisfying assignment, false otherwise.
/// Call when the number of variables is large (> 10).
pub fn solve_long(expr: &Expression, symbol_table: &SymbolTable) -> bool {
    let num_vars = symbol_table.vars.len();
    for assignment in 0..(1 << num_vars) {
        if expr.eval(symbol_table, assignment) {
            return true;
        }
    }
    false
}
