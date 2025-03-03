//! Equation solving for Sierra generation.
pub mod expr;

use std::fmt::Debug;
use std::hash::Hash;

use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
pub use expr::Expr;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
use indexmap::map::Entry;

/// Solving a set of equations and returning the values of the symbols contained in them.
/// # Arguments
/// * `equations` - The equations to solve.
/// * `minimization_vars` - Vars to minimize - in ranked ordering - first minimize the sum of the
///   first set - then the sum of the second and so on.
/// # Returns
/// * `Some(OrderedHashMap<Var, i64>)` - The solutions to the equations.
/// * `None` - The equations are unsolvable.
pub fn try_solve_equations<Var: Clone + Debug + PartialEq + Eq + Hash>(
    mut equations: Vec<Expr<Var>>,
    minimization_vars: Vec<Vec<Var>>,
) -> Option<OrderedHashMap<Var, i64>> {
    let (final_iter, high_rank_iters) = minimization_vars.split_last()?;
    // Iterating over the non-last minimization var layers.
    for target_vars in high_rank_iters {
        let layer_solution = try_solve_equations_iteration(&equations, target_vars)?;
        // Add constraints for the solution that we found for the target variables.
        for v in target_vars {
            let value = *layer_solution.get(v).unwrap();
            equations.push(Expr::from_const(value.try_into().unwrap()) - Expr::from_var(v.clone()));
        }
    }
    try_solve_equations_iteration(&equations, final_iter)
}

/// Solving a set of equations and returning the values of the symbols contained in them.
/// # Arguments
/// * `equations` - The equations to solve.
/// * `target_vars` - Minimize the sum of those variables.
/// # Returns
/// * `Some(OrderedHashMap<Var, i64>)` - The solutions to the equations.
/// * `None` - The equations are unsolvable.
fn try_solve_equations_iteration<Var: Clone + Debug + PartialEq + Eq + Hash>(
    equations: &[Expr<Var>],
    target_vars: &[Var],
) -> Option<OrderedHashMap<Var, i64>> {
    let mut vars = variables!();
    let mut orig_to_solver_var = OrderedHashMap::default();
    // Add all variables to structure and map.
    for eq in equations {
        for var in eq.var_to_coef.keys() {
            match orig_to_solver_var.entry(var.clone()) {
                Entry::Occupied(_) => {}
                Entry::Vacant(e) => {
                    e.insert(vars.add(variable().min(0).name(format!("{var:?}"))));
                }
            }
        }
    }
    let target: Expression = target_vars.iter().map(|v| *orig_to_solver_var.get(v).unwrap()).sum();

    let mut problem = vars.minimise(target).using(default_solver);
    // Adding constraints for all equations.
    for eq in equations.iter() {
        let as_solver_expr = |expr: &Expr<Var>| {
            Expression::from_other_affine(expr.const_term)
                + expr
                    .var_to_coef
                    .iter()
                    .map(|(var, coef)| (*coef as i32) * *orig_to_solver_var.get(var).unwrap())
                    .sum::<Expression>()
        };
        problem = problem.with(as_solver_expr(eq).eq(Expression::from_other_affine(0)));
    }
    let solution = problem.solve().ok()?;
    Some(
        orig_to_solver_var
            .into_iter()
            .map(|(orig, solver)| (orig, solution.value(solver).round() as i64))
            .collect(),
    )
}
