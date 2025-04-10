// // Equals rule for sets
use conjure_core::ast::{Atom, DeclarationKind, Domain, Expression, Literal, SymbolTable};
use conjure_core::metadata::Metadata;
use conjure_core::rule_engine::{
    register_rule, ApplicationError::RuleNotApplicable, ApplicationResult,
};

use std::rc::Rc;
use Expression::*;

use crate::ast::{Declaration, SetAttr};
use crate::rule_engine::Reduction;
use crate::matrix_expr;

#[register_rule(("Base", 8800))]
fn eq_to_subsetEq(expr: &Expression, _: &SymbolTable) -> ApplicationResult {
   match expr {
    // check a is a set
    // check b is a set
    // add atom 4 options
       Eq(_, a, b) => match (a.as_ref(), b.as_ref()) {
        (
            Expression::AbstractLiteral(m1, a),
            Expression::AbstractLiteral(m2, b),
        ) => {
            //let mut vecnew = Vec::new();
            let expr1 = Expression::AbstractLiteral(m1.clone(), a.clone());
            let expr2 = Expression::AbstractLiteral(m2.clone(), b.clone());
            let expr3 = SubsetEq(Metadata::new(), Box::new(expr1.clone()), Box::new(expr2.clone()));
            let expr4 = SubsetEq(Metadata::new(), Box::new(expr2.clone()), Box::new(expr1.clone()));

            // NIK
            Ok(Reduction::pure(And(Metadata::new(), Box::new(matrix_expr![expr3.clone(), expr4.clone()]))))
            //Expression::And(Metadata::new(),Box::new(into_matrix_expr![exprs]))
        }
       _ => Err(RuleNotApplicable),
        }
    _ => Err(RuleNotApplicable),
   }
}