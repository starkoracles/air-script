pub use air_script_core::{
    AccessType, ConstantBinding, ConstantValueExpr, Expression, Identifier, Iterable,
    ListComprehension, ListFolding, ListFoldingValueExpr, SymbolAccess, TraceAccess, TraceBinding,
    TraceSegment, VariableBinding, VariableValueExpr,
};
pub use parser::ast;
use std::collections::{BTreeMap, BTreeSet};

pub mod constraint_builder;
use constraint_builder::{ConstrainedBoundary, ConstraintBuilder};

pub mod constraints;
use constraints::{
    AlgebraicGraph, ConstraintDomain, ConstraintRoot, Constraints, Operation, CURRENT_ROW,
    MIN_CYCLE_LENGTH,
};
pub use constraints::{IntegrityConstraintDegree, NodeIndex};

pub mod declarations;
use declarations::Declarations;
pub use declarations::{PeriodicColumn, PublicInput};

mod symbol_table;
pub use symbol_table::Value;
use symbol_table::{Symbol, SymbolBinding, SymbolTable};

mod validation;
use validation::{SemanticError, SourceValidator};

#[cfg(test)]
mod tests;

// TYPE ALIASES
// ================================================================================================
pub type BoundaryConstraintsMap = BTreeMap<usize, Expression>;

// AIR IR
// ================================================================================================

/// Internal representation of an AIR.
///
/// TODO: docs
#[derive(Default, Debug)]
pub struct AirIR {
    pub air_name: String,
    pub declarations: Declarations,
    pub constraints: Constraints,
}

impl AirIR {
    // --- CONSTRUCTOR ----------------------------------------------------------------------------

    /// Consumes the provided source and generates a matching AirIR.
    pub fn new(source: ast::Source) -> Result<Self, SemanticError> {
        let ast::Source(source) = source;

        // set a default name.
        let mut air_name = String::from("CustomAir");

        // process the declarations of identifiers first, using a single symbol table to enforce
        // uniqueness.
        let mut symbol_table = SymbolTable::default();
        let mut validator = SourceValidator::new();
        let mut eval_exprs = Vec::new();
        let mut boundary_stmts = Vec::new();
        let mut integrity_stmts = Vec::new();

        for section in source {
            match section {
                ast::SourceSection::AirDef(Identifier(air_def)) => {
                    // update the name of the air.
                    air_name = air_def;
                }
                ast::SourceSection::Constant(constant) => {
                    symbol_table.insert_constant(constant)?;
                }
                ast::SourceSection::Trace(trace_bindings) => {
                    if !trace_bindings.is_empty() {
                        validator.exists("main_trace_columns");
                    }
                    if trace_bindings.len() > 1 {
                        validator.exists("aux_trace_columns");
                    }
                    // process & validate the trace bindings
                    symbol_table.insert_trace_bindings(trace_bindings)?;
                }
                ast::SourceSection::PublicInputs(inputs) => {
                    // process & validate the public inputs
                    symbol_table.insert_public_inputs(inputs)?;
                    validator.exists("public_inputs");
                }
                ast::SourceSection::PeriodicColumns(columns) => {
                    // process & validate the periodic columns
                    symbol_table.insert_periodic_columns(columns)?;
                }
                ast::SourceSection::RandomValues(values) => {
                    symbol_table.insert_random_values(values)?;
                    validator.exists("random_values");
                }
                ast::SourceSection::BoundaryConstraints(stmts) => {
                    // save the boundary statements for processing after the SymbolTable is built.
                    boundary_stmts.extend(stmts);
                    validator.exists("boundary_constraints");
                }
                ast::SourceSection::IntegrityConstraints(stmts) => {
                    // save the integrity statements for processing after the SymbolTable is built.
                    integrity_stmts.extend(stmts);
                    validator.exists("integrity_constraints");
                }
                ast::SourceSection::EvaluatorFunction(eval_expr) => eval_exprs.push(eval_expr),
            }
        }

        // validate sections
        validator.check()?;

        // process the variable & constraint statements, and validate them against the symbol table.

        // TODO: process evaluators

        // process the variable & constraint statements, and validate them against the symbol table.
        let mut constraint_builder = ConstraintBuilder::new(symbol_table);
        constraint_builder.insert_boundary_constraints(boundary_stmts)?;
        constraint_builder.insert_integrity_constraints(integrity_stmts)?;

        let (declarations, constraints) = constraint_builder.into_air();

        Ok(Self {
            air_name,
            declarations,
            constraints,
        })
    }

    // --- PUBLIC ACCESSORS FOR DECLARATIONS ------------------------------------------------------

    pub fn air_name(&self) -> &str {
        &self.air_name
    }

    pub fn constants(&self) -> &[ConstantBinding] {
        self.declarations.constants()
    }

    pub fn periodic_columns(&self) -> &[PeriodicColumn] {
        self.declarations.periodic_columns()
    }

    pub fn public_inputs(&self) -> &[PublicInput] {
        self.declarations.public_inputs()
    }

    pub fn trace_segment_widths(&self) -> &[u16] {
        self.declarations.trace_segment_widths()
    }

    // --- PUBLIC ACCESSORS FOR BOUNDARY CONSTRAINTS ----------------------------------------------

    pub fn num_boundary_constraints(&self, trace_segment: u8) -> usize {
        self.constraints.num_boundary_constraints(trace_segment)
    }

    pub fn boundary_constraints(&self, trace_segment: TraceSegment) -> &[ConstraintRoot] {
        self.constraints.boundary_constraints(trace_segment)
    }

    // --- PUBLIC ACCESSORS FOR INTEGRITY CONSTRAINTS ---------------------------------------------

    pub fn integrity_constraints(&self, trace_segment: TraceSegment) -> &[ConstraintRoot] {
        self.constraints.integrity_constraints(trace_segment)
    }

    pub fn integrity_constraint_degrees(
        &self,
        trace_segment: TraceSegment,
    ) -> Vec<IntegrityConstraintDegree> {
        self.constraints.integrity_constraint_degrees(trace_segment)
    }

    pub fn validity_constraints(&self, trace_segment: TraceSegment) -> Vec<&ConstraintRoot> {
        self.constraints
            .integrity_constraints(trace_segment)
            .iter()
            .filter(|constraint| matches!(constraint.domain(), ConstraintDomain::EveryRow))
            .collect()
    }

    pub fn transition_constraints(&self, trace_segment: TraceSegment) -> Vec<&ConstraintRoot> {
        self.constraints
            .integrity_constraints(trace_segment)
            .iter()
            .filter(|constraint| matches!(constraint.domain(), ConstraintDomain::EveryFrame(_)))
            .collect()
    }

    pub fn constraint_graph(&self) -> &AlgebraicGraph {
        self.constraints.graph()
    }
}
